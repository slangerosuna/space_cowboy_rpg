use serde::{Serialize, Deserialize};
use super::super::cognitive_modules::Plan;
use crate::utils::Rng;


#[derive(Serialize, Deserialize)]
pub struct Scratch {
    pub att_bandwidth: f32,
    pub retention: f32,
    pub gossip_threshold: f32,
    pub daily_plan: Plan,
    pub gossip: Vec<Gossip>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Gossip {
    pub content: String,
    pub interest: f32,
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

    pub fn get_random_gossip(&self, rng: &mut Rng) -> &Gossip {
        self.gossip.get(rng.range(0, self.gossip.len())).unwrap()
    }
}