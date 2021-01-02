use serde_derive::Deserialize;
use serde_derive::Serialize;

//use crate::game::pokedex::pokemon::Stat;

#[derive(Debug, Serialize, Deserialize)]
pub struct TomlPokemonConfig {
	
	pub pokedex_data: PokedexData,
	pub base_stats: Option<BaseStats>,
	pub moves: Option<Vec<LearnableMove>>,
	pub training: Option<Training>,
	pub breeding: Option<Breeding>,
	
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PokedexData {
	
	pub number: usize,
	pub name: String,
	pub primary_type: String,
	pub secondary_type: Option<String>,
	pub species: Option<String>,
	pub height: Option<f32>,
	pub weight: Option<f32>,
	
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseStats {
	
	pub hp: Option<usize>,
	pub atk: Option<usize>,
	pub def: Option<usize>,
	pub sp_atk: Option<usize>,
	pub sp_def: Option<usize>,
	pub speed: Option<usize>,
	
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LearnableMoves {
	pub items: Vec<LearnableMove>,
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