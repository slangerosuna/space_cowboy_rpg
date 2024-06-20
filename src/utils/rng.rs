use bevy::prelude::*;

const HASH: usize = 0x811c9dc5;

#[derive(Resource)]
pub struct Rng {
    state: usize,
}

impl Rng {
    pub fn new(seed: usize) -> Self {
        Self {
            state: seed,
        }
    }

    pub fn next(&mut self) -> usize {
        self.state = self.state.wrapping_mul(HASH);
        self.state
    }

    pub fn next_range(&mut self, min: usize, max: usize) -> usize {
        min + self.next() % (max - min)
    }
}