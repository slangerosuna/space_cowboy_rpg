use bevy::prelude::*;
use std::time::SystemTime;

mod one_shot_registry;
mod rng;
pub use one_shot_registry::*;
pub use rng::*;

pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Rng::new(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as usize,
        ))
        .init_resource::<OneShotRegistry>()
        .add_systems(Update, rng_system);
    }
}
