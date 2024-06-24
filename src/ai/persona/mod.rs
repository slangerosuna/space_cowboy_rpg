use bevy::prelude::*;
use serde::{Serialize, Deserialize};

mod cognitive_modules;
mod memory_structures;
mod skills;

pub use cognitive_modules::*;
use memory_structures::*;
use skills::*;

#[derive(Component, Serialize, Deserialize)]
pub struct Persona {
    pub name: String,
    pub age: u32,
    pub birthday: u32,
    pub associative_memory: AssociativeMemory,
    pub scratch: Scratch,
    pub skills: Skills,
    pub background: String,
    pub personality: Personality,
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