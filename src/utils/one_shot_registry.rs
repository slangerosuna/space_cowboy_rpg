use std::collections::HashMap;

use bevy::{ecs::system::SystemId, prelude::*};

#[derive(Resource, Deref)]
pub struct OneShotRegistry(HashMap<&'static str, SystemId>);

impl FromWorld for OneShotRegistry {
    fn from_world(world: &mut World) -> Self {
        let mut map = HashMap::new();

        map.insert(
            "crate::ai::persona::simulate_day",
            world.register_system(crate::ai::persona::simulate_day),
        );

        Self(map)
    }
}
