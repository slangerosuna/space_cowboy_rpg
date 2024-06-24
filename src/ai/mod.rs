use std::{future::Future, pin::Pin};

use bevy::prelude::*;
use rs_openai::{audio::{CreateTranscriptionRequestBuilder, Language}, shared::response_wrapper::OpenAIError, OpenAI};
use tokio::sync::Mutex;

use crate::Config;
use utils::audio::*;

mod persona;
mod utils;

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

#[derive(Resource)]
pub struct PlayerTranscriber {
    pub transcribe_player_fut: Option<Pin<Box<dyn Future<Output = Result<String, OpenAIError>> + Send + Sync>>>,
}

impl PlayerTranscriber {
    pub fn new() -> Self {
        Self { transcribe_player_fut: None }
    }

    pub fn transcribe_player(&mut self, open_ai: &OpenAPI, key_press_waiter: Mutex<()>) {
        // safe because the OpenAPI resource is never dropped or kept as a mutable reference
        let open_ai = unsafe { std::mem::transmute::<&OpenAPI, &'static OpenAPI>(open_ai) };
        self.transcribe_player_fut = Some(Box::pin(transcribe_player_internal(open_ai, key_press_waiter)));
    }
}

pub async fn transcribe_player_internal(open_ai: &OpenAPI, key_press_waiter: Mutex<()>) -> Result<String, OpenAIError> {
    let response = open_ai.client.audio().create_transcription_with_text_response(
        &(CreateTranscriptionRequestBuilder::default()
            .file(listen_to_player(key_press_waiter).await)
            .language(Language::English)
            .build()?)
    ).await?;

    Ok(response)
}