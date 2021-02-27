use serde::Deserialize;
use super::types::PokemonType;

pub mod instance;
pub mod serializable;

#[derive(Default, Debug, Clone, Deserialize)]
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