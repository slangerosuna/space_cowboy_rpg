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
    pub api_key: String,
    pub api_org: String,
}

fn main() {
    let config: Config = toml::from_str(include_str!("../config.toml")).unwrap();
    let api_key = config.api_key;
    let api_org = config.api_org;

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AiPlugin { api_key, api_org })
        .add_plugins(UtilPlugin)
        .add_plugins(RPGPlugin)
        .run();
}