use serde::{Serialize, Deserialize};
use super::super::ElevenLabs;

#[derive(Serialize, Deserialize)]
pub struct Voice {

}

impl Voice {
    pub fn tts(&self, text: &str, elevenlabs: &mut ElevenLabs) {
        //TODO: Implement TTS using ElevenLabs API
    }
}