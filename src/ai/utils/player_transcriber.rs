use crate::ai::OpenAPI;
use crate::RT;
use bevy::prelude::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::StreamConfig;
use futures::future::FutureExt;
use hound::WavSpec;
use ringbuf::{Consumer, HeapRb, SharedRb};
use rs_openai::audio::{CreateTranscriptionRequestBuilder, Language};
use rs_openai::{
    audio::ResponseFormat,
    shared::{response_wrapper::OpenAIError, types::FileMeta},
};
use std::{
    io::Cursor,
    mem::MaybeUninit,
    sync::{atomic::AtomicBool, Arc},
    task::Poll,
};
use tokio::task::JoinHandle;

#[derive(Resource)]
pub struct PlayerTranscriber {
    transcribe_player_handle: Option<JoinHandle<Result<String, OpenAIError>>>,
    key_press_waiter: Arc<AtomicBool>,
    mic_input: MicInput,
}

struct MicInput {
    consumer: Consumer<f32, Arc<SharedRb<f32, Vec<MaybeUninit<f32>>>>>,
}

pub fn consume_idle_mic_input(mut mic_input: ResMut<PlayerTranscriber>) {
    if mic_input.is_transcribing() {
        return;
    }

    loop {
        match mic_input.mic_input.consumer.pop() {
            Some(_) => {}
            _ => break,
        }
    }
}

impl PlayerTranscriber {
    pub fn new() -> Self {
        let microphone = cpal::default_host()
            .default_input_device()
            .expect("no input device available");
        let stream_config = microphone
            .default_input_config()
            .expect("no default input config available");
        let stream_config: StreamConfig = stream_config.into();

        let latency_frames = stream_config.sample_rate.0 as f32;
        let latency_samples = latency_frames as usize * stream_config.channels as usize;

        let ring = HeapRb::<f32>::new(latency_samples * 2);
        let (mut producer, consumer) = ring.split();

        let mic_input = MicInput { consumer };

        let input_stream = microphone
            .build_input_stream(
                &stream_config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    for &sample in data {
                        if producer.push(sample).is_err() {
                            eprintln!("ring buffer overrun");
                            break;
                        }
                    }
                },
                move |err| {
                    eprintln!("an error occurred on the input stream: {}", err);
                },
                None,
            )
            .unwrap();

        input_stream.play().unwrap();
        std::mem::forget(input_stream);

        Self {
            transcribe_player_handle: None,
            key_press_waiter: Arc::new(AtomicBool::new(false)),
            mic_input,
        }
    }

    pub fn transcribe_player(&mut self, open_ai: &OpenAPI, rt: &RT) {
        if self.is_transcribing() {
            return;
        }
        // safe because the OpenAPI resource is never dropped or kept as a mutable reference
        let open_ai = unsafe { std::mem::transmute::<&OpenAPI, &'static OpenAPI>(open_ai) };
        // safe as long as we ensure that there are no concurrent transcriptions, which we do
        // technically can make use of undefined behavior (multiple mutable references) when methods are called, but there are no side effects to this
        let this = unsafe { std::mem::transmute::<&mut Self, &'static mut Self>(self) };
        self.key_press_waiter
            .store(false, std::sync::atomic::Ordering::Relaxed);

        self.transcribe_player_handle = Some(
            rt.0.spawn(this.transcribe_player_internal(open_ai, this.key_press_waiter.clone())),
        );
    }

    pub fn poll(&mut self) -> Poll<Result<String, OpenAIError>> {
        if let Some(handle) = self.transcribe_player_handle.as_mut() {
            if handle.is_finished() {
                let res = handle.now_or_never();
                let res = res.unwrap();
                let res = Poll::Ready(res.unwrap());
                self.transcribe_player_handle = None;
                return res;
            }
        }

        Poll::Pending
    }

    pub fn press_key(&mut self) {
        self.key_press_waiter
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn is_transcribing(&self) -> bool {
        self.transcribe_player_handle.is_some()
    }

    async fn transcribe_player_internal(
        &mut self,
        open_ai: &'static OpenAPI,
        key_press_waiter: Arc<AtomicBool>,
    ) -> Result<String, OpenAIError> {
        let response = open_ai
            .client
            .audio()
            .create_transcription_with_text_response(
                &(CreateTranscriptionRequestBuilder::default()
                    .file(self.listen_to_player(key_press_waiter).await)
                    .language(Language::English)
                    .response_format(ResponseFormat::Text)
                    .build()?),
            )
            .await?;

        Ok(response)
    }

    async fn listen_to_player(&mut self, key_press_waiter: Arc<AtomicBool>) -> FileMeta {
        let mut buffer = Vec::new();

        while !key_press_waiter.load(std::sync::atomic::Ordering::Relaxed) {
            for _ in 0..4096 {
                match self.mic_input.consumer.pop() {
                    Some(sample) => buffer.push(sample),
                    _ => {
                        break;
                    }
                }
            }
        }

        let buffer = Self::to_wav(&mut buffer);

        FileMeta {
            buffer,
            filename: "player_audio.wav".to_string(),
        }
    }

    fn to_wav(buffer: &mut Vec<f32>) -> Vec<u8> {
        let mut wav = Cursor::new(Vec::new());
        let spec = WavSpec {
            channels: 1,
            sample_rate: 48000, //buffer.sample_rate() as u32,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        {
            let mut writer = hound::WavWriter::new(&mut wav, spec).unwrap();
            for sample in buffer {
                writer.write_sample(*sample).unwrap();
            }
            writer.finalize().unwrap();
        }

        wav.into_inner()
    }
}
