use super::super::cognitive_modules::Plan;
use super::{Association, AssociativeMemory, ConceptNode};
use crate::utils::Rng;
use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Component)]
pub struct Scratch {
    pub att_bandwidth: f32,
    pub retention: f32,
    pub gossip_threshold: f32,
    pub daily_plan: Plan,
    pub gossip: Vec<Gossip>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Gossip {
    // First word of the content is the subject of the gossip
    pub content: String,
    pub interest: f32,
}

impl Gossip {
    pub fn to_association(&self) -> Association {
        Association {
            concept1: ConceptNode {
                word: self
                    .content
                    .split_ascii_whitespace()
                    .next()
                    .unwrap()
                    .to_string(),
            },
            concept2: ConceptNode {
                word: self.content.clone(),
            },
            strength: self.interest,
        }
    }
}

impl Scratch {
    pub fn new() -> Self {
        Self {
            att_bandwidth: 3.0,
            retention: 5.0,
            gossip_threshold: 0.5,
            daily_plan: Plan::new(),
            gossip: Vec::new(),
        }
    }

    pub fn add_gossip(&mut self, gossip: Gossip) {
        self.gossip.push(gossip);
    }

    pub fn get_random_gossip(&self, rng: &Rng) -> &Gossip {
        self.gossip.get(rng.range(0, self.gossip.len())).unwrap()
    }

    pub fn forget_gossip(&mut self, rng: &Rng) {
        let mut series = rng.get_series();
        self.gossip
            .retain(|g| g.interest > 1.0 / self.retention - series.next().unwrap().f32() * 0.1);
    }

    pub fn fade_gossip(&mut self, rng: &Rng) {
        let mut series = rng.get_series();
        self.gossip
            .iter_mut()
            .for_each(|g| g.interest -= series.next().unwrap().f32() * 0.1);
    }

    pub fn store_to_memory(&self, memory: &mut AssociativeMemory, rng: &Rng) {
        let mut series = rng.get_series();
        self.gossip
            .iter()
            .filter(|g| g.interest > 1.0 / self.retention - series.next().unwrap().f32() * 0.3)
            .map(|g| g.to_association())
            .filter(|a| {
                memory
                    .get_association(&a.concept1)
                    .unwrap_or(Vec::new())
                    .iter()
                    .all(|ma| ma.concept2.word != a.concept2.word)
                    && memory
                        .get_association(&a.concept2)
                        .unwrap_or(Vec::new())
                        .iter()
                        .all(|ma| ma.concept1.word != a.concept1.word)
            })
            .collect::<Vec<_>>()
            .drain(..)
            .for_each(|a| memory.add_association(a));
    }
}
