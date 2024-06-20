use crate::ai::{persona::*, Ai};
use crate::utils::Rng;

impl Persona {
    pub fn converse_with_persona(&mut self, other: &mut Persona, rng: &mut Rng) {
        let gossip = self.scratch.get_random_gossip(rng);
        if gossip.interest > self.scratch.gossip_threshold {
            let gossip = (*gossip).clone();
            other.scratch.add_gossip(gossip);
        }

        let gossip = other.scratch.get_random_gossip(rng);
        if gossip.interest > other.scratch.gossip_threshold {
            let gossip = (*gossip).clone();
            self.scratch.add_gossip(gossip);
        }
    }

    pub fn converse_with_player(&mut self, ai: &mut Ai, rng: &mut Rng) {
        unimplemented!() //TODO: Implement
    }
}