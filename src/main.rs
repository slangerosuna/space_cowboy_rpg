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
