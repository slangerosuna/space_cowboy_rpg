use bevy::prelude::*;

const HASH: usize = 0x811c9dc5;

#[derive(Resource, Clone)]
pub struct Rng {
    state: usize,
}

impl Rng {
    pub fn new(seed: usize) -> Self {
        Self {
            state: seed,
        }
    }

    pub fn mutate_state(&mut self) {
        self.state = self.state.wrapping_mul(HASH);
    }

    pub fn range(&self, min: usize, max: usize) -> usize {
        min + self.next() % (max - min)
    }

    /**
     * Returns a random f32 between 0.0 and 1.0
     */
    pub fn f32(&self) -> f32 {
        self.next() as f32 / std::usize::MAX as f32
    }

    pub fn next(&self) -> usize {
        self.state.wrapping_mul(HASH)
    }

    pub fn bool(&self) -> bool {
        self.next() % 2 == 0
    }

    pub fn get_series(&self) -> Box<dyn Iterator<Item = Rng>> {
        let mut clone = self.clone();
        Box::new(std::iter::from_fn(move || { clone.mutate_state(); Some(clone.clone()) }))
    }
}

pub fn rng_system(mut rng: ResMut<Rng>) {
    rng.mutate_state();
}
