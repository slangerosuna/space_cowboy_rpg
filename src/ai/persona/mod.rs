use bevy::prelude::*;
use cognitive_modules::converse::ConversationHandler;
use serde::{Deserialize, Serialize};

pub mod cognitive_modules;
pub mod memory_structures;
mod persona_gen;
pub mod skills;
pub mod voice;

pub use memory_structures::*;
pub use persona_gen::*;
use skills::*;
use voice::*;

use crate::utils::Rng;

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
    pub race: String,
    pub name: String,
    pub age: u32,
    pub birthday: u32,
    pub skills: Skills,
    pub background: String,
    pub personality: Personality,
    pub voice: Voice,

    pub traits: Vec<String>,
    pub ideals: Vec<String>,
    pub bonds: Vec<String>,
    pub flaws: Vec<String>,

    #[serde(skip)]
    pub conversation_handler: ConversationHandler,
}

impl Persona {
    pub fn new(
        race: String,
        name: String,
        age: u32,
        birthday: u32,
        skills: Skills,
        background: String,
        personality: Personality,
        voice: Voice,
        traits: Vec<String>,
        ideals: Vec<String>,
        bonds: Vec<String>,
        flaws: Vec<String>,
    ) -> Self {
        Self {
            race,
            name,
            age,
            birthday,
            skills,
            background,
            personality,
            voice,
            traits,
            ideals,
            bonds,
            flaws,
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
}

impl Personality {
    pub fn new(
        openness: f32,
        conscientiousness: f32,
        extraversion: f32,
        agreeableness: f32,
        neuroticism: f32,
    ) -> Self {
        Self {
            openness,
            conscientiousness,
            extraversion,
            agreeableness,
            neuroticism,
        }
    }

    pub fn new_random(rng: &Rng) -> Self {
        let mut series = rng.get_series();
        Self {
            openness: series.next().unwrap().f32(),
            conscientiousness: series.next().unwrap().f32(),
            extraversion: series.next().unwrap().f32(),
            agreeableness: series.next().unwrap().f32(),
            neuroticism: series.next().unwrap().f32(),
        }
    }
}

fn trait_to_str(val: f32, low: &str, mid: &str, high: &str) -> String {
    if val < 0.3 {
        low.to_string()
    } else if val < 0.7 {
        mid.to_string()
    } else {
        high.to_string()
    }
}

impl Personality {
    fn as_string(&self) -> String {
        let mut personality = String::new();

        personality.push_str(&trait_to_str(
            self.openness,
            "closed-minded, ",
            "",
            "open-minded",
        ));
        personality.push_str(&trait_to_str(
            self.conscientiousness,
            "disorganized, ",
            "",
            "organized",
        ));
        personality.push_str(&trait_to_str(
            self.extraversion,
            "introverted, ",
            "",
            "extroverted",
        ));
        personality.push_str(&trait_to_str(
            self.agreeableness,
            "antagonistic, ",
            "",
            "agreeable",
        ));
        personality.push_str(&trait_to_str(
            self.neuroticism,
            "emotionally stable",
            "",
            "emotionally unstable",
        ));

        personality
    }
}
