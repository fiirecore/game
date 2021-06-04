
use super::{Effective, PokemonType};

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

    #[deprecated]
    pub const fn effective_old(&self, pokemon_type: PokemonType) -> Effective {
        // match category {
        //     MoveCategory::Status => todo!("status move effectiveness"),
        //     _ => {
        //         #[cfg(feature = "logging")]
        //         deps::log::warn!("To - do: change pokemon type effective functions");
                if self.supereffective(&pokemon_type) {
                    Effective::SuperEffective
                } else if self.noteffective(&pokemon_type) {
                    Effective::NotEffective
                } else if self.ineffective(pokemon_type) {
                    Effective::Ineffective
                } else {
                    Effective::Effective
                }
            // },
        // }
        
    }

    pub const fn supereffective(&self, pokemon_type: &PokemonType) -> bool {
        match self {

            Self::Unknown => false,

            Self::Normal => false,

            Self::Fire => match pokemon_type {
                Self::Grass => true,
                Self::Ice => true,
                Self::Bug => true,
                Self::Steel => true,
                _ => false,
            }

            Self::Water => match pokemon_type {
                Self::Fire => true,
                Self::Ground => true,
                Self::Rock => true,
                _ => false,
            }

            Self::Electric => match pokemon_type {
                Self::Water => true,
                Self:: Flying => true,
                _ => false,
            }

            Self::Grass => match pokemon_type {
                Self::Water => true,
                Self::Ground => true,
                Self::Rock => true,
                _ => false,
            }
            
            Self::Ice => match pokemon_type {
                Self::Grass => true,
                Self::Ground => true,
                Self::Flying => true,
                Self::Dragon => true,
                _ => false,
            }

            Self::Fighting => match pokemon_type {
                Self::Normal => true,
                Self::Ice => true,
                Self::Rock => true,
                Self::Dark => true,
                Self::Steel => true,
                _ => false,
            }
            
            Self::Poison => match pokemon_type {
                Self::Grass => true,
                Self::Fairy => true,
                _ => false,
            }
            
            Self::Ground => match pokemon_type {
                Self::Fire => true,
                Self::Electric => true,
                Self::Poison => true,
                Self::Rock => true,
                Self::Steel => true,
                _ => false,
            }
            
            Self::Flying => match pokemon_type {
                Self::Grass => true,
                Self::Fighting => true,
                Self::Bug => true,
                _ => false,
            }
            
            Self::Psychic => match pokemon_type {
                Self::Fighting => true,
                Self::Poison => true,
                _ => false,
            }
            
            Self::Bug => match pokemon_type {
                Self::Grass => true,
                Self::Psychic => true,
                Self::Dark => true,
                _ => false,
            }
            
            Self::Rock => match pokemon_type {
                Self::Fire => true,
                Self::Ice => true,
                Self::Flying => true,
                Self::Bug => true,
                _ => false,
            }
            
            Self::Ghost => match pokemon_type {
                Self::Psychic => true,
                Self::Ghost => true,
                _ => false,
            }
            
            Self::Dragon => match pokemon_type {
                Self::Dragon => true,
                _ => false,
            }
            
            Self::Dark => match pokemon_type {
                Self::Psychic => true,
                Self::Ghost => true,
                _ => false,
            }

            Self::Steel => match pokemon_type {
                Self::Ice => true,
                Self::Rock => true,
                Self::Fairy => true,
                _ => false,
            }

            Self::Fairy => match pokemon_type {
                Self::Fighting => true,
                Self::Dragon => true,
                Self::Dark => true,
                _ => false,
            }

        }
    }

    pub const fn noteffective(&self, pokemon_type: &PokemonType) -> bool {

        match self {

            Self::Unknown => false,

            Self::Normal => match pokemon_type {
                _ => false,
            }
            Self::Fire => match pokemon_type {
                Self::Grass => true,
                Self::Ice => true,
                Self::Bug => true,
                Self::Steel => true,
                _ => false,
            }
            Self::Water => match pokemon_type {
                Self::Water => true,
                Self::Grass => true,
                Self::Dragon => true,
                _ => false,
            }
            Self::Electric => match pokemon_type {
                Self::Electric => true,
                Self::Grass => true,
                Self::Dragon => true,
                _ => false,
            }
            Self::Grass => match pokemon_type {
                Self::Fire => true,
                Self::Grass => true,
                Self::Poison => true,
                Self::Flying => true,
                Self::Bug => true,
                Self::Dragon => true,
                Self::Steel => true,
                _ => false,
            }
            Self::Ice => match pokemon_type {
                Self::Fire => true,
                Self::Water => true,
                Self::Ice => true,
                Self::Steel => true,
                _ => false,
            }
            Self::Fighting => match pokemon_type {
                Self::Poison => true,
                Self::Flying => true,
                Self::Psychic => true,
                Self::Bug => true,
                Self::Fairy => true,
                _ => false,
            }
            Self::Poison => match pokemon_type {
                Self::Poison |
                Self::Ground |
                Self::Rock |
                Self::Ghost => true,
                _ => false,
            }
            Self::Ground => match pokemon_type {
                Self::Grass |
                Self::Bug => true,
                _ => false,
            }
            Self::Flying => match pokemon_type {
                Self::Electric |
                Self::Rock |
                Self::Steel => true,
                _ => false,
            }
            Self::Psychic => match pokemon_type {
                Self::Psychic |
                Self::Steel => true,
                _ => false,
            }
            Self::Bug => match pokemon_type {
                Self::Fire |
                Self::Fighting |
                Self::Poison |
                Self::Flying |
                Self::Ghost |
                Self::Steel |
                Self::Fairy => true,
                _ => false,
            }
            Self::Rock => match pokemon_type {
                Self::Fighting |
                Self::Ground |
                Self::Steel => true,
                _ => false,
            }
            Self::Ghost => match pokemon_type {
                Self::Dark => true,
                _ => false,
            }
            Self::Dragon => match pokemon_type {
                Self::Steel => true,
                _ => false,
            }
            Self::Dark => match pokemon_type {
                Self::Fighting |
                Self::Dark |
                Self::Fairy => true,
                _ => false,
            }
            Self::Steel => match pokemon_type {
                Self::Fire |
                Self::Water |
                Self::Electric |
                Self::Steel => true,
                _ => false,
            }
            Self::Fairy => match pokemon_type {
                Self::Fire |
                Self::Poison |
                Self::Steel => true,
                _ => false,
            }
        }

    }

    pub const fn ineffective(&self, pokemon_type: PokemonType) -> bool {
        match self {
            Self::Normal => match pokemon_type {
                Self::Ghost => true,
                _ => false,
            }
            Self::Electric => match pokemon_type {
                Self::Ground => true,
                _ => false,
            }
            Self::Fighting => match pokemon_type {
                Self::Ghost => true,
                _ => false,
            }
            Self::Poison => match pokemon_type {
                Self::Steel => true,
                _ => false,
            }
            Self::Ground => match pokemon_type {
                Self::Flying => true,
                _ => false,
            }
            Self::Psychic => match pokemon_type {
                Self::Dark => true,
                _ => false,
            }
            Self::Ghost => match pokemon_type {
                Self::Normal => true,
                _ => false,
            },
            Self::Dragon => match pokemon_type {
                Self::Fairy => true,
                _ => false,
            }
            _ => false,
        }
    }

}