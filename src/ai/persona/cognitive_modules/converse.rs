use lazy_static::lazy_static;
use rs_openai::chat::{ChatCompletionMessageRequestBuilder, CreateChatRequestBuilder, Role};
use tokio::task::JoinHandle;

use crate::ai::utils::prompt_template::PromptTemplate;
use crate::ai::{persona::*, OpenAPI, PlayerTranscriber};
use crate::utils::Rng;
use crate::RT;
use std::sync::Mutex;

#[derive(Default)]
pub struct ConversationHandler {
    handle: Mutex<Option<JoinHandle<()>>>,
}

impl Persona {
    pub fn converse_with_persona(self_scratch: &Scratch, other_scratch: &Scratch, rng: &Rng) {
        let gossip = self_scratch.get_random_gossip(rng);
        if gossip.interest > self_scratch.gossip_threshold {
            other_scratch.add_gossip(gossip);
        }

        let gossip = other_scratch.get_random_gossip(rng);
        if gossip.interest > other_scratch.gossip_threshold {
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
            let player_transcriber = std::mem::transmute::<
                &PlayerTranscriber,
                &'static PlayerTranscriber,
            >(player_transcriber);
            let scratch = std::mem::transmute::<&Scratch, &'static Scratch>(scratch);
            let associative =
                std::mem::transmute::<&AssociativeMemory, &'static AssociativeMemory>(associative);
            let rng = std::mem::transmute::<&Rng, &'static Rng>(rng);

            let mut guard = self.conversation_handler.handle.lock().unwrap();
            *guard = Some(rt.spawn(this.converse_with_player(
                open_api,
                player_transcriber,
                scratch,
                associative,
                rng,
            )));
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
        let mut req = CreateChatRequestBuilder::default()
            .model("gpt-3.5-turbo")
            .messages(vec![ChatCompletionMessageRequestBuilder::default()
                .role(Role::System)
                .content(format!(
                    "{}\n{}\n{}\n{}\n{}\n{}\n{}",
                    APPROPRIATE_CONTEXT.format(vec![]),
                    EMOTIONAL_EXPRESSION.format(vec![]),
                    END_CONVERSATION.format(vec![]),
                    INCLUDE_QUERIES.format(vec![]),
                    RELATIONSHIP.format(vec![&scratch.relationship.clone().into()]),
                    self.format_who_i_am(scratch, rng),
                    get_string(
                        &associative
                            .find_association_in_text("player")
                            .iter()
                            .map(|a| a.clone().into())
                            .collect()
                    )
                ))
                .build()
                .unwrap()])
            .build()
            .unwrap();
        'a: loop {
            let response = player_transcriber
                .transcribe_player_async(open_api)
                .await
                .unwrap();

            let associations = associative.find_association_in_text(&response);
            let associations = get_string(&associations.iter().map(|a| a.clone().into()).collect());

            let response = PLAYER_RESPONSE.format(vec![&response, &associations]);

            req.messages.push(
                ChatCompletionMessageRequestBuilder::default()
                    .role(Role::User)
                    .content(response)
                    .build()
                    .unwrap(),
            );

            'b: loop {
                let response = open_api.client.chat().create(&req).await.unwrap();

                req.messages.push(
                    ChatCompletionMessageRequestBuilder::default()
                        .role(Role::Assistant)
                        .content(response.choices[0].message.content.clone())
                        .build()
                        .unwrap(),
                );

                let response = response.choices[0].message.content.as_str();

                macro_rules! vocalize {
                    () => {
                        //remove words in all caps
                        let response = response
                            .split_ascii_whitespace()
                            .filter(|s| !s.chars().all(char::is_uppercase))
                            .collect::<Vec<&str>>()
                            .join(" ");

                        self.voice.tts(response.as_str()).await.unwrap();
                    };
                }

                match response {
                    x if x.contains("QUERY:") => {
                        let query = response.split(":").collect::<Vec<&str>>()[1].trim();
                        let response = associative.find_association_in_text(query);
                        let response = QUERY_RESPONSE.format(vec![
                            &query.to_string(),
                            &get_string(&response.iter().map(|a| a.clone().into()).collect()),
                        ]);
                        req.messages.push(
                            ChatCompletionMessageRequestBuilder::default()
                                .role(Role::System)
                                .content(response)
                                .build()
                                .unwrap(),
                        );

                        continue 'b;
                    }
                    x if x.contains("END") => {
                        vocalize!();
                        break 'a;
                    }
                    _ => {
                        vocalize!();
                        break 'b;
                    }
                };
            }
        }
    }

    fn format_who_i_am(&self, scratch: &Scratch, rng: &Rng) -> String {
        WHO_I_AM.format(vec![
            &self.name,
            &self.age.to_string(),
            &self.background,
            &self.personality.as_string(),
            &get_string(&self.traits),
            &get_string(&self.ideals),
            &get_string(&self.bonds),
            &get_string(&self.flaws),
            &scratch.get_random_gossip(rng).content,
            &self.race,
        ])
    }
}

fn get_string(vec: &Vec<String>) -> String {
    vec[..].iter().fold(String::new(), |acc, s| acc + s + ", ")
}

const PROMPT_TEMPLATES_PATH: &str = "resources/prompt_templates/";

lazy_static! {
    static ref APPROPRIATE_CONTEXT: PromptTemplate = load_prompt_template("appropriate_context.txt");
    static ref EMOTIONAL_EXPRESSION: PromptTemplate = load_prompt_template("emotional_expression.txt");
    static ref END_CONVERSATION: PromptTemplate = load_prompt_template("end_conversation.txt");
    static ref INCLUDE_QUERIES: PromptTemplate = load_prompt_template("include_queries.txt");
    static ref PLAYER_RESPONSE: PromptTemplate = load_prompt_template("player_response.txt");
    static ref QUERY_RESPONSE: PromptTemplate = load_prompt_template("query_response.txt");
    static ref RELATIONSHIP: PromptTemplate = load_prompt_template("relationship.txt");
    static ref WHO_I_AM: PromptTemplate = load_prompt_template("who_i_am.txt");
}

fn load_prompt_template(file_name: &str) -> PromptTemplate {
    let path = format!("{}{}", PROMPT_TEMPLATES_PATH, file_name);
    PromptTemplate::load_file(&path)
}
