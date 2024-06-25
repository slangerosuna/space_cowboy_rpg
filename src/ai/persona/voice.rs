use std::{collections::HashMap, path::Path};

use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use rten::Model;
use rten_tensor::{
    prelude::*,
    { NdTensor, NdTensorView }
};

#[derive(Serialize, Deserialize)]
struct ModelConfig {
    audio: AudioConfig,
    inference: InferenceConfig,

    phoneme_id_map: HashMap<char, Vec<i32>>, // index is the char casted to usize
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            audio: AudioConfig::default(),
            inference: InferenceConfig::default(),

            phoneme_id_map: HashMap::new(),
        }
    }
}

fn phonemes_to_ids(phonemes: &str, config: &ModelConfig) -> NdTensor<i32, 1> {
    let start_ids = config
        .phoneme_id_map
        .get(&'^')
        .unwrap();
    let end_ids = config
        .phoneme_id_map
        .get(&'$')
        .unwrap();

    let replacement = [];
    let separator = [0];
    let mut ids: Vec<i32> = start_ids.to_vec();

    ids.extend(phonemes.chars().flat_map(|c| {
        if let Some(ids) = config.phoneme_id_map.get(&c) {
            ids.iter().chain(separator.iter())
        } else {
            println!("Phoneme not found: {}", c);
            replacement.iter().chain(separator.iter())
        }
    }));

    ids.extend(end_ids);
    NdTensor::from_vec(ids)
}

#[derive(Serialize, Deserialize)]
struct AudioConfig {
    sample_rate: i32,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: 41000,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct InferenceConfig {
    noise_scale: f32,
    length_scale: f32,
    noise_w: f32,
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self {
            noise_scale: 0.1,
            length_scale: 0.1,
            noise_w: 0.1,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Voice {

}

#[derive(Resource)]
pub struct VoiceResource {
    model: Model,
    config: ModelConfig,
}

impl VoiceResource {
    pub fn new(model: String, config: String) -> Self {
        let model: Vec<u8> = std::fs::read(model).unwrap();
        let model = Model::load(model).unwrap();

        let config_json = std::fs::read_to_string(config).unwrap();
        let config: ModelConfig = serde_json::from_str(&config_json).unwrap();

        Self {
            model,
            config,
        }
    }

    fn audio_float_to_int16(
        audio: NdTensorView<f32, 1>,
        max_wav_value: Option<f32>,
    ) -> NdTensor<i16, 1> {
        let max_wav_value = max_wav_value.unwrap_or(32767.0);
        let audio_max = audio
            .iter()
            .map(|x| x.abs())
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or(0.)
            .max(0.01);
        audio.map(|x| {
            let sample = x * (max_wav_value / audio_max);
            sample.clamp(-max_wav_value, max_wav_value) as i16
        })
    }

    pub fn get_audio(&self, phonemes: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>>{
        let phoneme_ids = phonemes_to_ids(phonemes, &self.config);
        let phoneme_ids_len = phoneme_ids.size(0);
        let phoneme_ids = phoneme_ids.into_shape([1, phoneme_ids_len]);
        let input_lengths = NdTensor::from([phoneme_ids_len as i32]);
        let scales = NdTensor::from([
            self.config.inference.noise_scale,
            self.config.inference.length_scale,
            self. config.inference.noise_w
        ]);

        let input_id = self.model.find_node("input").unwrap();
        let input_lengths_id = self.model.find_node("input_lengths").unwrap();
        let output_id = self.model.find_node("output").unwrap();
        let scales_id = self.model.find_node("scales").unwrap();

        let [samples] = self.model.run_n(
            &[
                (input_id, phoneme_ids.into()),
                (input_lengths_id, input_lengths.into()),
                (scales_id, scales.into()),
            ],
            [output_id],
            None,
        )?;
        let samples: NdTensor<f32, 4> = samples.try_into()?; // (batch, time, 1, sample)

        Ok(samples.into_shape([samples.size(1) as usize]).iter().map(|s| *s).collect())
    }
}

impl Voice {
    pub async fn tts(&self, text: &str) {
        let phonemes = "ðɪs ɪz ɐ tˈɛkst tə spˈiːtʃ sˈɪstəm.";

    }
}