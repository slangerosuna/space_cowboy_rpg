use crate::ai::{persona::*, OpenAPI};
use crate::utils::Rng;

impl Persona {
    pub fn converse_with_persona(
        self_scratch: &mut Scratch,
        other_scratch: &mut Scratch,
        rng: &mut Rng,
    ) {
        let gossip = self_scratch.get_random_gossip(rng);
        if gossip.interest > self_scratch.gossip_threshold {
            let gossip = (*gossip).clone();
            other_scratch.add_gossip(gossip);
        }

        let gossip = other_scratch.get_random_gossip(rng);
        if gossip.interest > other_scratch.gossip_threshold {
            let gossip = (*gossip).clone();
            self_scratch.add_gossip(gossip);
        }
    }

    pub async fn generate_response(
        &mut self,
        statement: String,
        open_api: &OpenAPI,
        rng: &Rng,
    ) -> String {
        unimplemented!() //TODO: Implement
    }
}
