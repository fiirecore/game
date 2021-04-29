extern crate firecore_dependencies as deps;

use deps::hash::HashMap;

use {
	pokemon::{PokemonId, Pokemon},
	moves::{MoveId, PokemonMove},
	item::{ItemId, Item},
};

pub mod pokemon;
pub mod moves;
pub mod item;

pub mod serialize;

pub type Pokedex = HashMap<PokemonId, Pokemon>;
pub type Movedex = HashMap<MoveId, PokemonMove>;
pub type Itemdex = HashMap<ItemId, Item>;

pub static mut POKEDEX: Option<Pokedex> = None;
pub static mut MOVEDEX: Option<Movedex> = None;
pub static mut ITEMDEX: Option<Itemdex> = None;

pub fn pokedex() -> &'static Pokedex {
	unsafe { POKEDEX.as_ref().expect("Pokedex was not initialized!") }
}

pub fn movedex() -> &'static Movedex {
	unsafe { MOVEDEX.as_ref().expect("Movedex was not initialized!") }
}

pub fn itemdex() -> &'static Itemdex {
	unsafe { ITEMDEX.as_ref().expect("Itemdex was not initialized!") }
}

pub fn new() {
	unsafe {
		POKEDEX = Some(HashMap::new());
		MOVEDEX = Some(HashMap::new());
		ITEMDEX = Some(HashMap::new());
	}
}