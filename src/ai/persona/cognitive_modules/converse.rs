use rs_openai::chat::{ChatCompletionMessageRequestBuilder, CreateChatRequestBuilder, Role};
use tokio::task::JoinHandle;

use crate::ai::{persona::*, OpenAPI, PlayerTranscriber};
use crate::utils::Rng;
use crate::RT;
use std::sync::Mutex;

#[derive(Default)]
pub struct ConversationHandler {
    handle: Mutex<Option<JoinHandle<()>>>,
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
        &self,
        open_api: &OpenAPI,
        player_transcriber: &PlayerTranscriber,
        scratch: &Scratch,
        associative: &AssociativeMemory,
        rt: &RT,
        rng: &Rng,
    ) {
        unsafe {
            let this = std::mem::transmute::<&Self, &'static Self>(self);
            let open_api = std::mem::transmute::<&OpenAPI, &'static OpenAPI>(open_api);
            let player_transcriber = std::mem::transmute::<&PlayerTranscriber, &'static PlayerTranscriber>(player_transcriber);
            let scratch = std::mem::transmute::<&Scratch, &'static Scratch>(scratch);
            let associative = std::mem::transmute::<&AssociativeMemory, &'static AssociativeMemory>(associative);
            let rng = std::mem::transmute::<&Rng, &'static Rng>(rng);

            let mut guard = self.conversation_handler.handle.lock().unwrap();
            *guard = Some(rt.0.spawn(this.converse_with_player(open_api, player_transcriber, scratch, associative, rng)));
        }
    }

    async fn converse_with_player(
        &self,
        open_api: &OpenAPI,
        player_transcriber: &PlayerTranscriber,
        scratch: &Scratch,
        associative: &AssociativeMemory,
        rng: &Rng,
    ) {
        let response = player_transcriber.transcribe_player_async(open_api).await.unwrap();

        let req = CreateChatRequestBuilder::default()
            .model("gpt-3.5-turbo")
            .messages(vec![ChatCompletionMessageRequestBuilder::default()
                .role(Role::User)
                .content(response)
                .build().unwrap()])
            .build().unwrap();

        let response = open_api.client.chat().create(&req).await.unwrap();

        self.voice.tts(&response.choices[0].message.content.as_str()).await.unwrap();
    }
}
