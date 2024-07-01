#![feature(async_closure)]
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use game::GamePlugin;
use serde::{Deserialize, Serialize};
use std::env::set_var;
use tokio::runtime::Runtime;
use toml;

mod ai;
mod game;
mod gen;
mod networking;
mod rpg;
mod utils;

use ai::AiPlugin;
use rpg::RPGPlugin;
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
    unsafe {
        set_var("ELEVEN_API_KEY", config.elevenlabs_key.as_str());
    }

    let runtime = Runtime::new().unwrap();

    App::new()
        .add_systems(Startup, test)
        .insert_resource(RT(runtime))
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(AiPlugin::from_config(config))
        .add_plugins(GamePlugin)
        .add_plugins(UtilPlugin)
        .add_plugins(RPGPlugin)
        .run();

    std::process::exit(0);
}

fn test(
    rt: Res<RT>,
    rng: Res<utils::Rng>,
    player_transcriber: Res<ai::utils::player_transcriber::PlayerTranscriber>,
    open_api: Res<ai::OpenAPI>,
) {
    let persona = Box::new(ai::persona::Persona::new(
        "Clyde".to_string(),
        20,
        0,
        ai::persona::skills::Skills {
            innate: vec![],
            learned: vec![],
        },
        "Folk Hero".to_string(),
        ai::persona::Personality::new_random(&rng),
        ai::persona::voice::Voice {
            voice_id: "Clyde".to_string(),
        },
        vec!["easily bored".to_string()],
        vec!["Respect".to_string()],
        vec!["deeply attached to your home".to_string()],
        vec!["sometimes too trusting".to_string()],
    ));
    let mut scratch = Box::new(ai::persona::memory_structures::Scratch::new());
    scratch.add_gossip(ai::persona::memory_structures::Gossip {
        content: "Rachel is a lesbian".to_string(),
        interest: 0.5,
    });

    let associative = Box::new(ai::persona::memory_structures::AssociativeMemory::new());

    persona.start_conversation_with_player(
        &open_api,
        &player_transcriber,
        &scratch,
        &associative,
        &rt,
        &rng,
    );

    std::mem::forget(persona);
    std::mem::forget(scratch);
    std::mem::forget(associative);
}
