#![feature(async_closure)]
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use tokio::runtime::Runtime;
use toml;
use std::env::set_var;

mod ai;
mod gen;
mod utils;
mod rpg;

use rpg::RPGPlugin;
use ai::AiPlugin;
use utils::UtilPlugin;

#[derive(Resource)]
pub struct RT(Runtime);

#[derive(Serialize, Deserialize)]
struct Config {
    pub openapi_key: String,
    pub elevenlabs_key: String,
}

fn main() {
    let config: Config = toml::from_str(include_str!("../config.toml")).unwrap();
    unsafe { set_var("ELEVEN_API_KEY", config.elevenlabs_key.as_str() ); }

    let runtime = Runtime::new().unwrap();

    App::new()
        .insert_resource(RT(runtime))
        .add_systems(Startup, test)
        .add_plugins(DefaultPlugins)
        .add_plugins(AiPlugin::from_config(config))
        .add_plugins(UtilPlugin)
        .add_plugins(RPGPlugin)
        .run();

    std::process::exit(0);
}

use ai::persona::voice::Voice;
use ai::utils::player_transcriber::PlayerTranscriber;
use ai::OpenAPI;
use std::task::Poll;
 use rs_openai::{
     chat::{ChatCompletionMessageRequestBuilder, CreateChatRequestBuilder, Role},
     OpenAI,
 };

fn test(openapi: Res<OpenAPI>, rt: Res<RT>, mut player_transcriber: ResMut<PlayerTranscriber>) {
    player_transcriber.transcribe_player(&openapi, &rt);
    println!("recording");
    std::thread::sleep(std::time::Duration::from_secs(10));
    player_transcriber.press_key(&rt);

    let response = loop {
        if let Poll::Ready(response) = player_transcriber.poll() {
            break response;
        }
    }.unwrap();

    let req = CreateChatRequestBuilder::default()
        .model("gpt-3.5-turbo")
        .messages(vec![ChatCompletionMessageRequestBuilder::default()
            .role(Role::User)
            .content(response)
            .build().unwrap()])
        .build().unwrap();

    let response = rt.0.block_on(openapi.client.chat().create(&req)).unwrap();

    let voice = Voice { voice_id: "Clyde".to_string() };

    rt.0.block_on(voice.tts(&response.choices[0].message.content.as_str())).unwrap();
}
