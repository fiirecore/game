use serde::{Deserialize, Serialize};
use deps::{
	hash::HashMap,
	// tinystr::TinyStr16,
};

use super::pokemon::types::PokemonType;

pub type Movedex = HashMap<MoveId, PokemonMove>;

pub static mut MOVEDEX: Option<Movedex> = None;

pub fn movedex() -> &'static Movedex {
	unsafe { MOVEDEX.as_ref().expect("Movedex was not initialized!") }
}

pub mod saved;
pub mod instance;

pub mod target;
pub mod script;
pub mod persistent;

pub type MoveId = u16; // change to tinystr16
pub type Power = u8;
pub type Accuracy = u8;
pub type PP = u8;

pub type MoveRef = &'static PokemonMove;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PokemonMove {

	pub id: MoveId,

	pub name: String,
	pub category: MoveCategory,
	#[serde(rename = "type")]
	pub pokemon_type: PokemonType,

	pub power: Option<Power>,
	pub accuracy: Option<Accuracy>,
	pub pp: PP,

	#[serde(default)]
	pub target: target::MoveTarget,

	pub script: Option<script::MoveScript>,
	
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