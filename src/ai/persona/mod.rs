use bevy::prelude::*;
use cognitive_modules::converse::ConversationHandler;
use serde::{Deserialize, Serialize};

mod cognitive_modules;
pub mod memory_structures;
mod skills;
pub mod voice;

use memory_structures::*;
use skills::*;
use voice::*;

pub fn simulate_day(
    mut persona_query: Query<(&mut Persona, &mut Scratch, &mut AssociativeMemory)>,
    mut rng: ResMut<crate::utils::Rng>,
) {
    for (persona, mut scratch, mut associative_memory) in persona_query.iter_mut() {
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

    #[serde(skip)]
    pub conversation_handler: ConversationHandler,
}

impl Persona {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            age: 0,
            birthday: 0,
            skills: Skills::new(),
            background: String::new(),
            personality: Personality {
                openness: 0.0,
                conscientiousness: 0.0,
                extraversion: 0.0,
                agreeableness: 0.0,
                neuroticism: 0.0,
                traits: Vec::new(),
                ideals: Vec::new(),
                bonds: Vec::new(),
                flaws: Vec::new(),
            },
            voice: Voice { voice_id: "Clyde".to_string() },
            conversation_handler: ConversationHandler::default(),
        }
    }

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
