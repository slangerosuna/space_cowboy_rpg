use bevy::prelude::*;
use serde::{Serialize, Deserialize};

mod cognitive_modules;
mod memory_structures;
mod skills;
pub mod voice;

pub use cognitive_modules::*;
use memory_structures::*;
use skills::*;
use voice::*;

pub fn simulate_day(
    mut persona_query: Query<(&mut Persona, &mut Scratch, &mut AssociativeMemory)>,
    mut rng: ResMut<crate::utils::Rng>,
) {
    for (mut persona, mut scratch, mut associative_memory) in persona_query.iter_mut() {
        scratch.forget_gossip(&mut rng);
        scratch.fade_gossip(&mut rng);
        scratch.store_to_memory(&mut associative_memory, &mut rng);
    }
}

#[derive(Component, Serialize, Deserialize)]
pub struct Persona {
    pub name: String,
    pub age: u32,
    pub birthday: u32,
    pub skills: Skills,
    pub background: String,
    pub personality: Personality,
    pub voice: Voice,
}
#[derive(Serialize, Deserialize)]
pub struct Personality {
    pub openness: f32,
    pub conscientiousness: f32,
    pub extraversion: f32,
    pub agreeableness: f32,
    pub neuroticism: f32,

    pub traits: Vec<String>,
    pub ideals: Vec<String>,
    pub bonds: Vec<String>,
    pub flaws: Vec<String>,
}
