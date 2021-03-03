use pokemon::Pokemon;
use dashmap::DashMap as HashMap;

use moves::PokemonMove;

pub mod pokemon;
pub mod data;
pub mod types;
pub mod moves;
pub mod instance;
pub mod party;
pub mod texture;

lazy_static::lazy_static! {
	pub static ref POKEDEX: HashMap<PokemonId, Pokemon> = HashMap::new();
	pub static ref MOVEDEX: HashMap<u16, PokemonMove> = HashMap::new();
}

pub type PokemonId = u16;
pub type Level = u8;
pub type Stat = u8;

pub type MoveId = u16;