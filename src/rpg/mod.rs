use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum ExperienceType {
    LightArmor = 0,
    MediumArmor = 1,
    HeavyArmor = 2,
    Marksmanship = 3,
    Swordsmanship = 4,
    Abjuration = 5,
    Transmutation = 6,
    Conjuration = 7,
    Divination = 8,
    Enchantment = 9,
    Evocation = 10,
    Illusion = 11,
    Necromancy = 12,
}

impl ExperienceType {
    pub fn is_martial(&self) -> bool {
        match self {
            ExperienceType::LightArmor | ExperienceType::MediumArmor | ExperienceType::HeavyArmor | ExperienceType::Marksmanship | ExperienceType::Swordsmanship => true,
            _ => false,
        }
    }
}

#[derive(Component, Serialize, Deserialize)]
pub struct RPG {
    level: u8,
    magic_level: u8,
    martial_level: u8,
    magic_experience: u16,
    martial_experience: u16,
    experience_types: [u16; 13],
    levels_experience_types: [u8; 13],
    health: f32,
    max_health: u16,
    mana: f32,
    max_mana: u16,
    stamina: f32,
    max_stamina: u16,
    strength: u8,
    dexterity: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,
    constitution: u8,
}

fn modifier(stat: u8) -> i8 {
    (stat as i8 - 10) / 2
}

#[inline] //inline function rather than macro to force evaluation of arguments only once, improving performance
fn max(a: i8, b: i8) -> i8 { if a > b { a } else { b } }
macro_rules! max {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => (max($x, max!($($y),+)));
}

impl RPG {
    pub fn new(
        strength: u8,
        dexterity: u8,
        intelligence: u8,
        wisdom: u8,
        charisma: u8,
        constitution: u8,
    ) -> Self {
        Self {
            level: 1,
            magic_level: 1,
            martial_level: 1,
            magic_experience: 0,
            martial_experience: 0,
            experience_types: [0; 13],
            levels_experience_types: [1; 13],
            health: (8 + modifier(constitution)) as f32,
            max_health: (8 + modifier(constitution)) as u16,
            mana: (8 + max!(modifier(intelligence), modifier(wisdom))) as f32,
            max_mana: (8 + max!(modifier(intelligence), modifier(wisdom))) as u16,
            stamina: (8 + modifier(dexterity)) as f32,
            max_stamina: (8 + modifier(dexterity)) as u16,
            strength,
            dexterity,
            intelligence,
            wisdom,
            charisma,
            constitution,
        }
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.max_health += (5 as i8 + modifier(self.constitution)) as u16;
    }

    pub fn level_up_magic(&mut self) {
        self.level_up();
        self.magic_experience -= Self::xp_needed_to_level_up(self.magic_level);
        self.max_mana += (5 as i8 + max!(modifier(self.intelligence), modifier(self.wisdom))) as u16;
    }

    pub fn level_up_martial(&mut self) {
        self.level_up();
        self.martial_experience -= Self::xp_needed_to_level_up(self.martial_level);
        self.max_stamina += (5 as i8 + modifier(self.dexterity)) as u16;
    }

    pub fn xp_needed_to_level_up(level: u8) -> u16 {
        100 * level as u16
    }

    pub fn recalculate_max_health(&mut self) {
        self.max_health = 3 + (5 as i8 + modifier(self.constitution)) as u16 * self.level as u16;
    }

    pub fn recalculate_max_mana(&mut self) {
        self.max_mana = 3 + (5 as i8 + max!(modifier(self.intelligence), modifier(self.wisdom))) as u16 * self.magic_level as u16;
    }

    pub fn recalculate_max_stamina(&mut self) {
        self.max_stamina = 3 + (5 as i8 + modifier(self.dexterity)) as u16 * self.martial_level as u16;
    }

    pub fn add_xp(&mut self, xp: u16, experience_type: ExperienceType) {
        if experience_type.is_martial() {
            self.martial_experience += xp;
            if self.martial_experience >= Self::xp_needed_to_level_up(self.martial_level)
              { self.level_up_martial(); }
        } else {
            self.magic_experience += xp;
            if self.magic_experience >= Self::xp_needed_to_level_up(self.magic_level)
              { self.level_up_magic(); }
        }
        let cur_level = self.levels_experience_types[experience_type as usize];

        self.experience_types[experience_type as usize] += xp;
        let cur_experience = self.experience_types[experience_type as usize];
        if cur_experience >= Self::xp_needed_to_level_up(cur_level) {
            self.experience_types[experience_type as usize] -= Self::xp_needed_to_level_up(cur_level);
            self.levels_experience_types[experience_type as usize] += 1;
        }
    }
}

pub struct RPGPlugin;

impl Plugin for RPGPlugin {
    fn build(&self, app: &mut App) {
        
    }
}