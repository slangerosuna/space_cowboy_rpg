use bevy::prelude::*;
use rs_openai::{
    chat::{ChatCompletionMessageRequestBuilder, CreateChatRequestBuilder, Role},
    OpenAI,
};

pub struct AiPlugin {
    pub api_key: String,
    pub api_org: String,
}

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        let api_key = self.api_key.clone();
        let api_org = self.api_org.clone();

        app.insert_resource(Ai::new(api_key, Some(api_org)));
    }
}

#[derive(Resource)]
pub struct Ai {
    pub client: OpenAI,
}

impl Ai {
    fn new(api_key: String, api_org: Option<String>) -> Self {
        let client = OpenAI::new(&OpenAI { api_key, org_id: api_org, });
        Self { client }
    }
}