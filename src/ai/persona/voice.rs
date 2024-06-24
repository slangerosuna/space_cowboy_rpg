use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Voice {
    name: String,
}

impl Voice {
    pub async fn tts(&self, text: &str) {

    }
}