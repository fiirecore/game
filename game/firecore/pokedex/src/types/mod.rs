use serde::{Deserialize, Serialize};

use crate::moves::MoveCategory;

mod effective;

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

impl PokemonType {

	pub const fn effective(&self, target: Self, category: MoveCategory) -> Effective {
		match category {
			MoveCategory::Status => Effective::Ineffective,
			_ => match self {
				Self::Unknown => Effective::Effective,
				Self::Normal => match target {
					Self::Ghost => Effective::Ineffective,
					Self::Rock => Effective::NotEffective,
					Self::Steel => Effective::NotEffective,
					_ => Effective::Effective,
				}
				_ => {
					Self::effective_old(&self, target)
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