use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Skills {
    pub innate: Vec<String>,
    pub learned: Vec<String>,
}