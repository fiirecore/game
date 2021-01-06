use std::fmt::Display;

use crate::game::pokedex::pokemon::pokemon::PokemonType;

use super::move_category::MoveCategory;
use super::move_instance::MoveInstance;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct PokemonMove {

	pub name: String,
	pub category: MoveCategory,
	pub pokemon_type: Option<PokemonType>,
	pub power: Option<usize>,
	pub accuracy: Option<u8>,
	pub pp: u8,
	
}

impl PokemonMove {

	pub fn empty() -> Self {
		Self {
			name: String::new(),
			category: MoveCategory::Status,
			pokemon_type: None,
			power: None,
			accuracy: None,
			pp: 0,
		}
	}

	pub fn to_instance(&self) -> MoveInstance {
		MoveInstance {
			move_instance: self.clone(),
			remaining_pp: self.pp
		}
	}

}

impl Display for PokemonMove {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}