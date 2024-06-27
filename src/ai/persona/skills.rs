use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Skills {
    pub innate: Vec<Skill>,
    pub learned: Vec<Skill>,
}

impl Skills {
    pub fn new() -> Self {
        Self {
            innate: Vec::new(),
            learned: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Skill {
    pub skill: String,
    pub level: f32,
}
