use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use toml;

mod ai;
mod gen;
mod utils;
mod rpg;

use rpg::RPGPlugin;
use ai::AiPlugin;
use utils::UtilPlugin;

#[derive(Serialize, Deserialize)]
struct Config {
    pub openapi_key: String,
    pub azure_key: String,
    pub elevenlabs_key: String,
}

fn main() {
    let config: Config = toml::from_str(include_str!("../config.toml")).unwrap();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AiPlugin::from_config(config))
        .add_plugins(UtilPlugin)
        .add_plugins(RPGPlugin)
        .run();
}