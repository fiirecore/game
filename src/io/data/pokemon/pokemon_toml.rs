use serde::{Deserialize, Serialize};

use super::PokemonType;
use super::StatSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonConfig {
	
	pub pokedex_data: PokedexData,
	pub base_stats: StatSet,
	pub moves: Vec<LearnableMove>,
	pub training: Option<Training>,
	pub breeding: Option<Breeding>,
	
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PokedexData {
	
	pub number: usize,
	pub name: String,
	pub primary_type: PokemonType,
	pub secondary_type: Option<PokemonType>,
	pub species: String,
	pub height: f32,
	pub weight: f32,
	
}

impl Default for PokedexData {
	fn default() -> Self {
		Self {
			number: 0,
			name: "None".to_string(),
			primary_type: PokemonType::Normal,
			secondary_type: None,
			species: "None".to_string(),
			
			height: 0f32,
			weight: 0f32,
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LearnableMove {
	pub level: u8,
	pub move_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Training {
	
	pub ev_yield: Option<(String, usize)>,
	pub catch_rate: Option<u8>,
	pub base_friendship: Option<u8>,
	pub base_exp: Option<usize>,
//	pub growth_rate:
	
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Breeding {
	
//	pub groups: Vec<EggGroup>,
//	pub gender: Option< // something for percentages
	pub cycles: Option<u8>,
	
}