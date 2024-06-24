use std::io::Cursor;

use hound::WavSpec;
use rs_openai::shared::types::FileMeta;
use fon::{stereo::Stereo32, Frame, Stream};

use tokio::sync::Mutex;
use wavy::{ Microphone, MicrophoneStream };

pub async fn listen_to_player(key_press_waiter: Mutex<()>) -> FileMeta {
    let mut microphone = Microphone::default();
    microphone.record::<Stereo32>().await; // Clears the buffer
    {
        let _guard = key_press_waiter.lock().await;
    }
    let buffer = microphone.record::<Stereo32>().await;
    let buffer = to_wav(buffer);

    FileMeta {
        buffer,
        filename: "player_audio.wav".to_string(),
    }
}

fn to_wav(buffer: MicrophoneStream<Stereo32>) -> Vec<u8> {
    let mut wav = Cursor::new(Vec::new());
    let spec = WavSpec {
        channels: 2,
        sample_rate: buffer.sample_rate().unwrap() as u32,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    {
        let mut writer = hound::WavWriter::new(&mut wav, spec).unwrap();
        for sample in buffer.into_iter() {
            let channels = sample.channels();

            let left: f32 = channels[0].into();
            let right: f32 = channels[1].into();

            writer.write_sample(left).unwrap();
            writer.write_sample(right).unwrap();
        }
    }

    wav.into_inner()
}