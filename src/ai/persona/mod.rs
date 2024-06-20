use bevy::prelude::*;
use serde::{Serialize, Deserialize};

mod cognitive_modules;
mod memory_structures;
mod skills;

use cognitive_modules::*;
use memory_structures::*;
use skills::*;

#[derive(Component, Serialize, Deserialize)]
pub struct Persona {
    pub name: String,
    pub associative_memory: AssociativeMemory,
    pub scratch: Scratch,
    pub skills: Skills,
}

impl Persona {
    pub fn new(name: String, innate_skills: Vec<String>) -> Self {
        Self {
            name,
            associative_memory: AssociativeMemory::new(),
            scratch: Scratch::new(),
            skills: Skills { innate: innate_skills.into_iter().map(|e| Skill { skill: e, level: 1.0 }).collect(), learned: Vec::new() },
        }
    }
}