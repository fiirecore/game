use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, Clone, Serialize, Deserialize)]
pub struct SavedPokemonMove {

	pub name: String,
	pub remaining_pp: u8,
	
}