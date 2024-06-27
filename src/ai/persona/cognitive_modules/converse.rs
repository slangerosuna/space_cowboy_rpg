use crate::ai::{persona::*, OpenAPI, PlayerTranscriber};
use crate::utils::Rng;
use crate::RT;

#[derive(Default)]
pub struct ConversationHandler {

}

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

    pub fn start_conversation_with_player(
        &mut self,
        open_api: &OpenAPI,
        player_transcriber: &mut PlayerTranscriber,
        scratch: &mut Scratch,
        associative: &mut AssociativeMemory,
        rt: &RT,
        rng: &Rng,
    ) {

    }
}
