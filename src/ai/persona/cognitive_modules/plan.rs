use serde::{Serialize, Deserialize};

use crate::ai::persona::Persona;

#[derive(Serialize, Deserialize)]
pub struct Plan {
    pub tasks: Vec<Task>,
}

impl Plan {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Location; // This is a placeholder for now

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub time: f32,
    pub location: Location,
}

impl Task {
    pub fn new(time: f32, location: Location) -> Self {
        Self {
            time,
            location,
        }
    }
}

impl Persona {
    pub fn plan(&mut self) {
        unimplemented!() //TODO: Implement
    }
}