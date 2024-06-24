use crate::ai::{persona::*, OpenAPI};
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

    pub async fn generate_response(&mut self, statement: String, open_api: &OpenAPI, rng: &Rng) -> String {
        unimplemented!() //TODO: Implement
    }
}