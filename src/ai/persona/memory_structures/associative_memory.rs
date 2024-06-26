use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

use lazy_static::lazy_static;
use word2vec::wordvectors::WordVector;

lazy_static! {
    pub static ref WORD2VEC: WordVector =
        WordVector::load_from_binary("resources/word2vec.bin").unwrap();
}

const MIN_ASSOCIATIVE_STRENGTH: f32 = 0.9;
#[derive(Serialize, Deserialize, PartialEq)]
pub struct ConceptNode {
    pub word: String,
}

#[derive(Serialize, Deserialize)]
pub struct Association {
    // concept 1 must be one word
    pub concept1: ConceptNode,
    // concept 2 may be any length
    pub concept2: ConceptNode,
    pub strength: f32,
}

#[derive(Serialize, Deserialize, Component)]
pub struct AssociativeMemory {
    pub associations: Vec<Association>,
}

impl AssociativeMemory {
    pub fn new() -> Self {
        Self {
            associations: Vec::new(),
        }
    }

    pub fn add_association(&mut self, association: Association) {
        self.associations.push(association);
    }

    pub fn get_association(&self, concept: &ConceptNode) -> Option<Vec<&Association>> {
        let concept_vec = WORD2VEC.get_vector(&concept.word)?;
        Some(
            self.associations
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
