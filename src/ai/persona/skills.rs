use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Skills {
    pub innate: Vec<Skill>,
    pub learned: Vec<Skill>,
}

#[derive(Serialize, Deserialize)]
pub struct Skill {
    pub skill: String,
    pub level: f32,
}