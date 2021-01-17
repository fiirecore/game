use serde::{Deserialize, Serialize};

pub mod moves;
pub mod pokemon;
pub mod pokemon_party;
pub mod saved_pokemon;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Pokemon {
	
	pub data: PokedexData,
	pub base: StatSet,
	pub moves: Vec<LearnableMove>,
	// pub training: Option<Training>,
	// pub breeding: Option<Breeding>,
	
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

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
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
	
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Breeding {
	
//	pub groups: Vec<EggGroup>,
//	pub gender: Option< // something for percentages
	pub cycles: Option<u8>,
	
}

#[derive(PartialEq, Eq, Deserialize, Serialize)]
pub enum Gender {
	
	None,
	Male,
	Female,
	
}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq, Deserialize, Serialize)]
pub enum PokemonType {
	
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

impl Default for PokemonType {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StatSet {

	pub hp: u8,
	pub atk: u8,
	pub def: u8,
	pub sp_atk: u8,
	pub sp_def: u8,
	pub speed: u8,

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LargeStatSet {

	pub hp: u16,
	pub atk: u16,
	pub def: u16,
	pub sp_atk: u16,
	pub sp_def: u16,
	pub speed: u16,

}

impl StatSet {

	pub fn iv_random(random: &mut oorandom::Rand32) -> Self {

		Self {
			hp: random.rand_range(0..32) as u8,
			atk: random.rand_range(0..32) as u8,
			def: random.rand_range(0..32) as u8,
			sp_atk: random.rand_range(0..32) as u8,
			sp_def: random.rand_range(0..32) as u8,
			speed: random.rand_range(0..32) as u8,
		}

	}

	pub fn uniform(stat: u8) -> Self {

		Self {
			hp: stat,
			atk: stat,
			def: stat,
			sp_atk: stat,
			sp_def: stat,
			speed: stat,
		}
		
	}

}