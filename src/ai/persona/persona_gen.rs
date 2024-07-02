use crate::utils::Rng;

use super::{AssociativeMemory, Persona, Scratch};

impl Persona {
    pub fn new_random(
        rng: &Rng,
    ) -> (Self, Scratch, AssociativeMemory) {
        let series = rng.get_series();
        unimplemented!("Persona::new_random")
    }
}