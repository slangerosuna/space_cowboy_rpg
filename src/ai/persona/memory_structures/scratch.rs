use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Scratch {
    pub att_bandwidth: f32,
    pub retention: f32,
    pub daily_plan_req: Vec<String>,
}

impl Scratch {
    pub fn new() -> Self {
        Self {
            att_bandwidth: 3.0,
            retention: 5.0,
            daily_plan_req: Vec::new(),
        }
    }
}