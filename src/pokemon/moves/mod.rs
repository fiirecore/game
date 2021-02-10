use serde::Deserialize;
use super::types::PokemonType;

pub mod instance;

#[derive(Default, Clone, Deserialize)]
pub struct PokemonMove {

	pub number: u16,
	pub name: String,
	pub category: MoveCategory,
	pub pokemon_type: Option<PokemonType>,
	pub power: Option<usize>,
	pub accuracy: Option<u8>,
	pub pp: u8,
	
}

impl PokemonMove {

    pub fn from_string(data: &str) -> Result<PokemonMove, toml::de::Error> {
        return toml::from_str(data);
    }

}

impl std::fmt::Display for PokemonMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub struct MoveInstance {
	
	pub move_instance: PokemonMove, // To - do: possibly change to number in case of global pokedex variable
	pub remaining_pp: u8,
	
}

impl MoveInstance {

	pub fn use_move(&mut self) -> PokemonMove {
		self.remaining_pp -= 1;
		self.move_instance.clone()
	}

}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum MoveCategory {
	
	Physical,
	Special,
	Status,	
	
}

impl Default for MoveCategory {
    fn default() -> Self {
        Self::Status
    }
}