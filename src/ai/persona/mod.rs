use bevy::prelude::*;
use serde::{Serialize, Deserialize};

mod cognitive_modules;
mod memory_structures;
mod skills;

use memory_structures::*;
use skills::Skills;

#[derive(Component, Serialize, Deserialize)]
pub struct Persona {
    pub name: String,
    pub spatial_memory: SpatialMemory,
    pub associative_memory: AssociativeMemory,
    pub scratch: Scratch,
    pub skills: Skills,
}

impl Persona {
    pub fn new(name: String, innate_skills: Vec<String>) -> Self {
        Self {
            name,
            spatial_memory: SpatialMemory::new(),
            associative_memory: AssociativeMemory::new(),
            scratch: Scratch::new(),
            skills: Skills { innate: innate_skills, learned: Vec::new() },
        }
    }
}