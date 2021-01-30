use serde::Deserialize;
use super::PokemonType;

pub mod instance;

#[derive(Hash, Clone, Deserialize)]
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

	// pub fn to_instance(&self) -> MoveInstance {
	// 	MoveInstance {
	// 		move_instance: self.clone(),
	// 		remaining_pp: self.pp
	// 	}
    // }

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
	
	pub move_instance: PokemonMove,
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