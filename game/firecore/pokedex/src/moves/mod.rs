use serde::{Deserialize, Serialize};
use self::battle_script::BattleActionScript;
use self::script::MoveScript;

use super::pokemon::types::PokemonType;

pub type MoveId = u16;
pub type Power = u8;
pub type Accuracy = u8;
pub type PP = u8;

pub type MoveRef = &'static PokemonMove;

pub mod saved;
pub mod instance;

pub mod script;
pub mod persistent;

pub mod battle_script;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PokemonMove {

	pub id: MoveId,

	pub name: String,
	pub category: MoveCategory,
	#[serde(rename = "type")]
	pub pokemon_type: PokemonType,

	pub power: Option<Power>,
	pub accuracy: Option<Accuracy>,
	pub pp: PP,

	pub script: Option<MoveScript>,

	pub battle_script: Option<BattleActionScript>,
	
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum MoveCategory {
	Physical,
	Special,
	Status,	
}

impl std::fmt::Display for PokemonMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}