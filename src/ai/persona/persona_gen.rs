use crate::utils::Rng;

use super::{AssociativeMemory, Persona, Scratch};

impl Persona {
    pub fn new_random(rng: &Rng) -> (Self, Scratch, AssociativeMemory) {
        let mut series = rng.get_series();

        let race = random_from_file(&series.next().unwrap(), "resources/persona_gen/races.txt");
        let gender = series.next().unwrap().f32();

        const CHANCE_ENBY: f32 = 0.1;
        let gender = match gender {
            x if x < (0.5 - CHANCE_ENBY / 2.0) => "femme",
            x if x < (1.0 - CHANCE_ENBY) => "masc",
            _ => "androgyne",
        };

        let formatted_race = race.to_lowercase().replace(" ", "_");
        let first_name = random_from_file(
            &series.next().unwrap(),
            &format!("resources/persona_gen/names/first/{gender}/{formatted_race}.txt",),
        );
        let last_name = random_from_file(
            &series.next().unwrap(),
            &format!("resources/persona_gen/names/last/{formatted_race}.txt",),
        );
        let last_name = match last_name.as_str() {
            "None" => None,
            _ => Some(last_name),
        };
        unimplemented!("Persona::new_random")
    }
}

fn random_from_file(rng: &Rng, file: &str) -> String {
    let file = std::fs::read_to_string(file).expect("Could not read file");
    let lines = file.lines().collect::<Vec<_>>();

    rng.choose(&lines).to_string()
}
