use bevy::prelude::*;
use rs_openai::OpenAI;

use crate::Config;

mod persona;
mod utils;

pub struct AiPlugin {
    pub openapi_key: String,
    pub openapi_org: Option<String>,
    pub azure_key: String,
    pub elevenlabs_key: String,
}

impl AiPlugin {
    pub fn from_config(config: Config) -> Self {
        Self {
            openapi_key: config.openapi_key,
            openapi_org: None,
            azure_key: config.azure_key,
            elevenlabs_key: config.elevenlabs_key,
        }
    }
}

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        let api_key = self.openapi_key.clone();
        let api_org = self.openapi_org.clone();

        app
            .insert_resource(OpenAPI::new(api_key, api_org))
            .insert_resource(ElevenLabs { key: self.elevenlabs_key.clone() })
            .add_systems(Update, play_elevenlabs);

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

#[derive(Resource)]
pub struct ElevenLabs {
    pub key: String,
}

fn play_elevenlabs(mut elevenlabs: ResMut<ElevenLabs>) {

}