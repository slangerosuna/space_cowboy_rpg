use serde::{Serialize, Deserialize};

use elevenlabs_rs::{ Speech, Result, };

#[derive(Serialize, Deserialize)]
pub struct Voice {
    pub voice_id: String,
}

impl Voice {
    pub async fn tts(&self, text: &str) -> Result<()> {
        let speech = Speech::new(
            text,
            self.voice_id.as_str(),
            "eleven_monolingual_v1",
            0,
        ).await?;

        speech.play()?;

        Ok(())
    }
}
