
use super::PokemonType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Effective {

    Ineffective,
    NotEffective,
    Effective,
    SuperEffective,

}

impl Effective {

    pub const fn multiplier(self) -> f32 {
        match self {
            Effective::Ineffective => 0.0,
            Effective::NotEffective => 0.5,
            Effective::Effective => 1.0,
            Effective::SuperEffective => 2.0,
        }
    }

}

impl core::fmt::Display for Effective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Effective::Ineffective => "ineffective",
            Effective::NotEffective => "not very effective",
            Effective::Effective => "effective",
            Effective::SuperEffective => "super effective",
        })
    }
}

impl std::ops::Mul for Effective {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Effective::Ineffective => Self::Ineffective,
            Effective::NotEffective => match rhs {
                Effective::SuperEffective => Self::Effective,
                Effective::Ineffective => Self::Ineffective,
                _ => Self::NotEffective,
            }
            Effective::Effective => rhs,
            Effective::SuperEffective => match rhs {
                Effective::NotEffective => Self::Effective,
                Effective::Ineffective => Self::Ineffective,
                _ => Self::SuperEffective,
            }
        }
    }
}

impl PokemonType {

    pub const fn effective(&self, pokemon_type: PokemonType) -> Effective {
        if self.supereffective(&pokemon_type) {
            Effective::SuperEffective
        } else if self.noteffective(&pokemon_type) {
            Effective::NotEffective
        } else if self.ineffective(pokemon_type) {
            Effective::Ineffective
        } else {
            Effective::Effective
        }
    }

    pub const fn supereffective(&self, pokemon_type: &PokemonType) -> bool {
        use PokemonType::*;

        match self {

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

            Electric => match pokemon_type {
                Water => true,
                Flying => true,
                _ => false,
            }

            Grass => match pokemon_type {
                Water => true,
                Ground => true,
                Rock => true,
                _ => false,
            }
            
            Ice => match pokemon_type {
                Grass => true,
                Ground => true,
                Flying => true,
                Dragon => true,
                _ => false,
            }

            Fighting => match pokemon_type {
                Normal => true,
                Ice => true,
                Rock => true,
                Dark => true,
                Steel => true,
                _ => false,
            }
            
            Poison => match pokemon_type {
                Grass => true,
                Fairy => true,
                _ => false,
            }
            
            Ground => match pokemon_type {
                Fire => true,
                Electric => true,
                Poison => true,
                Rock => true,
                Steel => true,
                _ => false,
            }
            
            Flying => match pokemon_type {
                Grass => true,
                Fighting => true,
                Bug => true,
                _ => false,
            }
            
            Psychic => match pokemon_type {
                Fighting => true,
                Poison => true,
                _ => false,
            }
            
            Bug => match pokemon_type {
                Grass => true,
                Psychic => true,
                Dark => true,
                _ => false,
            }
            
            Rock => match pokemon_type {
                Fire => true,
                Ice => true,
                Flying => true,
                Bug => true,
                _ => false,
            }
            
            Ghost => match pokemon_type {
                Psychic => true,
                Ghost => true,
                _ => false,
            }
            
            Dragon => match pokemon_type {
                Dragon => true,
                _ => false,
            }
            
            Dark => match pokemon_type {
                Psychic => true,
                Ghost => true,
                _ => false,
            }

            Steel => match pokemon_type {
                Ice => true,
                Rock => true,
                Fairy => true,
                _ => false,
            }

            Fairy => match pokemon_type {
                Fighting => true,
                Dragon => true,
                Dark => true,
                _ => false,
            }

        }
    }

    pub const fn noteffective(&self, pokemon_type: &PokemonType) -> bool {

        match self {
            PokemonType::Normal => match pokemon_type {
                PokemonType::Rock => true,
                PokemonType::Steel => true,
                _ => false,
            }
            PokemonType::Fire => match pokemon_type {
                PokemonType::Grass => true,
                PokemonType::Ice => true,
                PokemonType::Bug => true,
                PokemonType::Steel => true,
                _ => false,
            }
            PokemonType::Water => match pokemon_type {
                PokemonType::Water => true,
                PokemonType::Grass => true,
                PokemonType::Dragon => true,
                _ => false,
            }
            PokemonType::Electric => match pokemon_type {
                PokemonType::Electric => true,
                PokemonType::Grass => true,
                PokemonType::Dragon => true,
                _ => false,
            }
            PokemonType::Grass => match pokemon_type {
                PokemonType::Fire => true,
                PokemonType::Grass => true,
                PokemonType::Poison => true,
                PokemonType::Flying => true,
                PokemonType::Bug => true,
                PokemonType::Dragon => true,
                PokemonType::Steel => true,
                _ => false,
            }
            PokemonType::Ice => match pokemon_type {
                PokemonType::Fire => true,
                PokemonType::Water => true,
                PokemonType::Ice => true,
                PokemonType::Steel => true,
                _ => false,
            }
            PokemonType::Fighting => match pokemon_type {
                PokemonType::Poison => true,
                PokemonType::Flying => true,
                PokemonType::Psychic => true,
                PokemonType::Bug => true,
                PokemonType::Fairy => true,
                _ => false,
            }
            PokemonType::Poison => match pokemon_type {
                PokemonType::Poison |
                PokemonType::Ground |
                PokemonType::Rock |
                PokemonType::Ghost => true,
                _ => false,
            }
            PokemonType::Ground => match pokemon_type {
                PokemonType::Grass |
                PokemonType::Bug => true,
                _ => false,
            }
            PokemonType::Flying => match pokemon_type {
                PokemonType::Electric |
                PokemonType::Rock |
                PokemonType::Steel => true,
                _ => false,
            }
            PokemonType::Psychic => match pokemon_type {
                PokemonType::Psychic |
                PokemonType::Steel => true,
                _ => false,
            }
            PokemonType::Bug => match pokemon_type {
                PokemonType::Fire |
                PokemonType::Fighting |
                PokemonType::Poison |
                PokemonType::Flying |
                PokemonType::Ghost |
                PokemonType::Steel |
                PokemonType::Fairy => true,
                _ => false,
            }
            PokemonType::Rock => match pokemon_type {
                PokemonType::Fighting |
                PokemonType::Ground |
                PokemonType::Steel => true,
                _ => false,
            }
            PokemonType::Ghost => match pokemon_type {
                PokemonType::Dark => true,
                _ => false,
            }
            PokemonType::Dragon => match pokemon_type {
                PokemonType::Steel => true,
                _ => false,
            }
            PokemonType::Dark => match pokemon_type {
                PokemonType::Fighting |
                PokemonType::Dark |
                PokemonType::Fairy => true,
                _ => false,
            }
            PokemonType::Steel => match pokemon_type {
                PokemonType::Fire |
                PokemonType::Water |
                PokemonType::Electric |
                PokemonType::Steel => true,
                _ => false,
            }
            PokemonType::Fairy => match pokemon_type {
                PokemonType::Fire |
                PokemonType::Poison |
                PokemonType::Steel => true,
                _ => false,
            }
        }

    }

    pub const fn ineffective(&self, pokemon_type: PokemonType) -> bool {
        match self {
            PokemonType::Normal => match pokemon_type {
                PokemonType::Ghost => true,
                _ => false,
            }
            PokemonType::Electric => match pokemon_type {
                PokemonType::Ground => true,
                _ => false,
            }
            PokemonType::Fighting => match pokemon_type {
                PokemonType::Ghost => true,
                _ => false,
            }
            PokemonType::Poison => match pokemon_type {
                PokemonType::Steel => true,
                _ => false,
            }
            PokemonType::Ground => match pokemon_type {
                PokemonType::Flying => true,
                _ => false,
            }
            PokemonType::Psychic => match pokemon_type {
                PokemonType::Dark => true,
                _ => false,
            }
            PokemonType::Ghost => match pokemon_type {
                PokemonType::Normal => true,
                _ => false,
            },
            PokemonType::Dragon => match pokemon_type {
                PokemonType::Fairy => true,
                _ => false,
            }
            _ => false,
        }
    }

}