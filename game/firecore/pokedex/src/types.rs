use serde::{Deserialize, Serialize};

use crate::moves::MoveCategory;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)] // To - do: move module
pub enum PokemonType {

	Unknown,
	
	Normal,
	Fire,
	Water,
	Electric,
	Grass,
	Ice,
	Fighting,
	Poison,
	Ground,
	Flying,
	Psychic,
	Bug,
	Rock,
	Ghost,
	Dragon,
	Dark,
	Steel,
	Fairy,
	
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Effective {

    Effective,
    Ineffective,
    NotEffective,
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

impl PokemonType {

	pub const fn effective(&self, target: Self, category: MoveCategory) -> Effective {
		match category {
			MoveCategory::Status => Effective::Ineffective,
			_ => match self {

				Self::Unknown => Effective::Effective,

				Self::Normal => match target {
					Self::Ghost => Effective::Ineffective,
					Self::Rock | Self::Steel => Effective::NotEffective,
					_ => Effective::Effective,
				},
				Self::Fire => match target {
					Self::Grass | Self::Ice | Self::Bug | Self::Steel => Effective::SuperEffective,
					Self::Fire | Self::Water | Self::Rock | Self::Dragon => Effective::NotEffective,
					_ => Effective::Effective,
				},
				Self::Water => match target {
					Self::Fire | Self::Ground | Self::Rock => Effective::SuperEffective,
					Self::Water | Self::Grass | Self::Dragon => Effective::NotEffective,
					_ => Effective::Effective,
				},
				Self::Electric => match target {
					Self::Water | Self::Flying => Effective::SuperEffective,
					Self::Electric | Self::Grass | Self::Dragon => Effective::NotEffective,
					Self::Ground => Effective::Ineffective,
					_ => Effective::Effective,
				},
				Self::Grass => match target {
					Self::Water | Self::Ground | Self::Rock => Effective::SuperEffective,
					Self::Fire | Self::Grass | Self::Poison | Self::Flying | Self::Bug | Self::Dragon | Self::Steel => Effective::NotEffective,
					_ => Effective::Effective,
				},
				Self::Ice => match target {
					Self::Grass |Self::Ground | Self::Flying | Self::Dragon => Effective::SuperEffective,
					Self::Fire | Self::Water | Self::Ice | Self::Steel => Effective::NotEffective,
					_ => Effective::Effective,
				},
				Self::Fighting => match target {
					Self::Normal | Self::Ice | Self::Rock | Self::Dark | Self::Steel => Effective::SuperEffective,
					Self::Poison | Self::Flying | Self::Psychic | Self::Bug | Self::Fairy => Effective::NotEffective,
					Self::Ghost => Effective::Ineffective,
					_ => Effective::Effective,
				},
				Self::Poison => match target {
					Self::Grass | Self::Fairy => Effective::SuperEffective,
					Self::Poison | Self::Ground | Self::Rock | Self::Ghost => Effective::NotEffective,
					Self::Steel => Effective::Ineffective,
					_ => Effective::Effective,
				},
				Self::Ground => match target {
					Self::Fire | Self::Electric | Self::Poison | Self::Rock | Self::Steel => Effective::SuperEffective,
					Self::Grass | Self::Bug => Effective::NotEffective,
					Self::Flying => Effective::Ineffective,
					_ => Effective::Effective,
				},
				Self::Flying => match target {
					Self::Grass | Self::Fighting | Self::Bug => Effective::SuperEffective,
					Self::Electric | Self::Rock | Self::Steel => Effective::NotEffective,
					_ => Effective::Effective,
				},
				Self::Psychic => match target {
					Self::Fighting | Self::Poison => Effective::SuperEffective,
					Self::Psychic | Self::Steel => Effective::NotEffective,
					Self::Dark => Effective::Ineffective,
					_ => Effective::Effective,
				},
				Self::Bug => match target {
					Self::Grass | Self::Psychic | Self::Dark => Effective::SuperEffective,
					Self::Fire | Self::Fighting | Self::Poison | Self::Flying | Self::Ghost | Self::Steel | Self::Fairy => Effective::NotEffective,
					_ => Effective::Effective,
				},
				Self::Rock => match target {
					Self::Fire | Self::Ice | Self::Flying | Self::Bug => Effective::SuperEffective,
					Self::Fighting | Self::Ground | Self::Steel => Effective::NotEffective,
					_ => Effective::Effective,
				},
				Self::Ghost => match target {
					Self::Psychic | Self::Ghost => Effective::SuperEffective,
					Self::Dark => Effective::NotEffective,
					Self::Normal => Effective::Ineffective,
					_ => Effective::Effective,
				},
				Self::Dragon => match target {
					Self::Dragon => Effective::SuperEffective,
					Self::Steel => Effective::NotEffective,
					Self::Fairy => Effective::Ineffective,
					_ => Effective::Effective,
				},
				Self::Dark => match target {
					Self::Psychic | Self::Ghost => Effective::SuperEffective,
					Self::Fighting | Self::Dark | Self::Fairy => Effective::NotEffective,
					_ => Effective::Effective,
				},
				Self::Steel => match target {
					Self::Ice | Self::Rock | Self::Fairy => Effective::SuperEffective,
					Self::Fire | Self::Water | Self::Electric | Self::Steel => Effective::NotEffective,
					_ => Effective::Effective,
				},
				Self::Fairy => match target {
					Self::Fighting | Self::Dragon | Self::Dark => Effective::SuperEffective,
					Self::Fire | Self::Poison | Self::Steel => Effective::NotEffective,
					_ => Effective::Effective,
				},
			}
		}
	}

}

impl Default for PokemonType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Default for Effective {
    fn default() -> Self {
        Self::Effective
    }
}

impl std::ops::Mul for Effective {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Self::Ineffective => Self::Ineffective,
            Self::NotEffective => match rhs {
                Self::SuperEffective => Self::Effective,
                Self::Ineffective => Self::Ineffective,
                _ => Self::NotEffective,
            }
            Self::Effective => rhs,
            Self::SuperEffective => match rhs {
                Self::NotEffective => Self::Effective,
                Self::Ineffective => Self::Ineffective,
                _ => Self::SuperEffective,
            }
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