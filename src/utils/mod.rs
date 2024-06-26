use bevy::prelude::*;
use std::time::SystemTime;

mod rng;
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
        .add_systems(Update, rng_system);
    }
}
