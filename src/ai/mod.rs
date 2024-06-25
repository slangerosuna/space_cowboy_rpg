use bevy::prelude::*;
use rs_openai::OpenAI;

use crate::Config;
use utils::player_transcriber::*;

mod persona;
pub mod utils;

pub struct AiPlugin {
    pub openapi_key: String,
    pub openapi_org: Option<String>,
}

impl AiPlugin {
    pub fn from_config(config: Config) -> Self {
        Self {
            openapi_key: config.openapi_key,
            openapi_org: None,
        }
    }
}

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        let api_key = self.openapi_key.clone();
        let api_org = self.openapi_org.clone();

        app
            .add_systems(Update, consume_idle_mic_input)
            .insert_resource(OpenAPI::new(api_key, api_org))
            .insert_resource(PlayerTranscriber::new());
    }
}

#[derive(Resource)]
pub struct OpenAPI {
    pub client: OpenAI,
}

impl OpenAPI {
    fn new(api_key: String, api_org: Option<String>) -> Self {
        let client = OpenAI::new(&OpenAI { api_key, org_id: api_org, });
        Self { client }
    }
}
