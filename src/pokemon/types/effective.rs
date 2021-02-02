use super::PokemonType;

#[derive(Debug)]
pub enum Effective {

    Ineffective,
    NotEffective,
    Effective,
    SuperEffective,

}

impl Effective {

    pub fn multiplier(self) -> f32 {
        match self {
            Effective::Ineffective => 0.0,
            Effective::NotEffective => 0.5,
            Effective::Effective => 1.0,
            Effective::SuperEffective => 2.0,
        }
    }

}

impl PokemonType {

    pub fn effective(&self, pokemon_type: PokemonType) -> f32 {
        if self.supereffective(pokemon_type) {
            macroquad::prelude::info!("{:?} is supereffective on {:?}", self, pokemon_type);
            Effective::SuperEffective.multiplier()
        } else {
            Effective::Effective.multiplier()
        }
    }

    pub fn supereffective(&self, pokemon_type: PokemonType) -> bool {
        use PokemonType::*;
        match *self {
            Normal => false,

            Fire => match pokemon_type {
                Grass => true,
                Ice => true,
                Bug => true,
                Steel => true,
                _ => false,
            }

            Water => match pokemon_type {
                Fire => true,
                Ground => true,
                Rock => true,
                _ => false,
            }

            PokemonType::Electric => match pokemon_type {
                _ => false,
            }

            Grass => match pokemon_type {
                Water => true,
                Ground => true,
                Rock => true,
                _ => false,
            }
            PokemonType::Ice => match pokemon_type {
                _ => false,
            }
            PokemonType::Fighting => match pokemon_type {
                _ => false,
            }
            PokemonType::Poison => match pokemon_type {
                _ => false,
            }
            PokemonType::Ground => match pokemon_type {
                _ => false,
            }
            PokemonType::Flying => match pokemon_type {
                _ => false,
            }
            PokemonType::Psychic => match pokemon_type {
                _ => false,
            }
            PokemonType::Bug => match pokemon_type {
                _ => false,
            }
            PokemonType::Rock => match pokemon_type {
                _ => false,
            }
            PokemonType::Ghost => match pokemon_type {
                _ => false,
            }
            PokemonType::Dragon => match pokemon_type {
                _ => false,
            }
            PokemonType::Dark => match pokemon_type {
                _ => false,
            }
            PokemonType::Steel => match pokemon_type {
                _ => false,
            }
            PokemonType::Fairy => match pokemon_type {
                _ => false,
            }
        }
    }

}