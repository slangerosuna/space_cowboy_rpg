use std::sync::Mutex;

use bevy::{prelude::Component, text};
use serde::{Deserialize, Serialize};

use lazy_static::lazy_static;
use word2vec::wordvectors::WordVector;

lazy_static! {
    static ref WORD2VEC: WordVector =
        WordVector::load_from_binary("resources/word2vec.bin").unwrap();
}

const MIN_ASSOCIATIVE_STRENGTH: f32 = 0.9;
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct ConceptNode {
    pub word: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Association {
    // concept 1 must be one word
    pub concept1: ConceptNode,
    // concept 2 may be any length
    pub concept2: ConceptNode,
    pub strength: f32,
}

impl Into<String> for Association {
    fn into(self) -> String {
        format!(
            "You associate \"{}\" with \"{}\" {}",
            self.concept1.word,
            self.concept2.word,
            match self.strength {
                x if x > 0.9 => "very strongly",
                x if x > 0.7 => "strongly",
                x if x > 0.5 => "moderately",
                _ => "weakly",
            }
        )
    }
}

#[derive(Serialize, Deserialize, Component)]
pub struct AssociativeMemory {
    pub associations: Mutex<Vec<Association>>,
}

impl AssociativeMemory {
    pub fn new() -> Self {
        Self {
            associations: Mutex::new(Vec::new()),
        }
    }

    pub fn add_association(&self, association: Association) {
        self.associations.lock().unwrap().push(association);
    }

    pub fn get_association(&self, concept: &ConceptNode) -> Option<Vec<Association>> {
        let concept_vec = WORD2VEC.get_vector(&concept.word)?;
        Some(
            self.associations
                .lock()
                .unwrap()
                .iter()
                .filter_map(|a| {
                    if Self::get_associative_strength_with_already_loaded_vectors(
                        concept_vec,
                        WORD2VEC.get_vector(&a.concept1.word)?,
                    ) > MIN_ASSOCIATIVE_STRENGTH
                        || Self::get_associative_strength_with_already_loaded_vectors(
                            concept_vec,
                            WORD2VEC.get_vector(&a.concept2.word)?,
                        ) > MIN_ASSOCIATIVE_STRENGTH
                    {
                        Some(a)
                    } else {
                        None
                    }
                })
                .map(|a| (*a).clone())
                .collect(),
        )
    }

    pub fn get_associative_strength(
        &self,
        concept1: &ConceptNode,
        concept2: &ConceptNode,
    ) -> Option<f32> {
        let concept1_vec = WORD2VEC.get_vector(&concept1.word)?;
        let concept2_vec = WORD2VEC.get_vector(&concept2.word)?;

        let word_association =
            Self::get_associative_strength_with_already_loaded_vectors(concept1_vec, concept2_vec);

        let memory_association = self
            .associations
            .lock()
            .unwrap()
            .iter()
            .filter_map(|a| {
                let a_concept1_vec = WORD2VEC.get_vector(&a.concept1.word)?;
                let a_concept2_vec = WORD2VEC.get_vector(&a.concept2.word)?;

                if Self::get_associative_strength_with_already_loaded_vectors(
                    a_concept1_vec,
                    concept1_vec,
                ) > MIN_ASSOCIATIVE_STRENGTH
                    && Self::get_associative_strength_with_already_loaded_vectors(
                        a_concept2_vec,
                        concept2_vec,
                    ) > MIN_ASSOCIATIVE_STRENGTH
                    || Self::get_associative_strength_with_already_loaded_vectors(
                        a_concept1_vec,
                        concept2_vec,
                    ) > MIN_ASSOCIATIVE_STRENGTH
                        && Self::get_associative_strength_with_already_loaded_vectors(
                            a_concept2_vec,
                            concept1_vec,
                        ) > MIN_ASSOCIATIVE_STRENGTH
                {
                    Some(a.strength)
                } else {
                    None
                }
            })
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        match memory_association {
            Some(memory_association) => {
                if memory_association > word_association {
                    Some(memory_association)
                } else {
                    Some(word_association)
                }
            }
            None => Some(word_association),
        }
    }

    fn get_associative_strength_with_already_loaded_vectors(
        concept1_vec: &Vec<f32>,
        concept2_vec: &Vec<f32>,
    ) -> f32 {
        let mut sum = 0.0;
        for i in 0..concept1_vec.len() {
            sum += concept1_vec[i] * concept2_vec[i];
        }
        return sum;
    }
}
use std::collections::HashSet;
lazy_static! {
    static ref IGNORABLE: HashSet<String> =
        std::fs::read_to_string("resources/misc/ignorable.txt")
            .unwrap()
            .lines()
            .map(|s| s.to_string())
            .collect();
}
impl AssociativeMemory {
    pub fn find_association_in_text(&self, text: &str) -> Vec<Association> {
        let tokens = text.split_whitespace().filter(|t| !IGNORABLE.contains(*t));
        let mut associations: Vec<Association> = Vec::new();

        for token in tokens {
            let concept = ConceptNode {
                word: token.to_string(),
            };
            if let Some(associations_with_concept) = self.get_association(&concept) {
                associations.extend(associations_with_concept);
            }
        }

        associations
    }
}
