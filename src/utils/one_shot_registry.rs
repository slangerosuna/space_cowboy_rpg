use std::collections::HashMap;

use bevy::{ecs::system::SystemId, prelude::*};

#[derive(Resource)]
pub struct OneShotRegistry(HashMap<String, SystemId>);

impl FromWorld for OneShotRegistry {
    fn from_world(world: &mut World) -> Self {
        let mut map = HashMap::new();

        map.insert(
            "simulate_day".into(),
            world.register_system(crate::ai::persona::simulate_day),
        );

        Self(map)
    }
}
