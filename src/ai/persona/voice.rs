use serde::{Deserialize, Serialize};

use elevenlabs_rs::{Result, Speech};

#[derive(Serialize, Deserialize)]
pub struct Voice {
    pub voice_id: String,
}

impl Voice {
    pub async fn tts(&self, text: &str) -> Result<()> {
        let speech = Speech::new(text, self.voice_id.as_str(), "eleven_turbo_v2", 0).await?;

        speech.play()?;

        Ok(())
    }
}