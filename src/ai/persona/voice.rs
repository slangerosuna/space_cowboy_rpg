use serde::{Deserialize, Serialize};

use elevenlabs_rs::{Result, Speech};

#[derive(Serialize, Deserialize)]
pub struct Voice {
    pub voice_id: String,
}

impl Voice {
    pub async fn tts(&self, text: &str) -> Result<()> {
        let speech = Speech::new(text, self.voice_id.as_str(), "eleven_monolingual_v1", 0).await?;

        speech.play()?;

        Ok(())
    }
}
