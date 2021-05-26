use deps::hash::HashMap;
use super::{Pokemon, PokemonId};

pub type Pokedex = HashMap<PokemonId, Pokemon>;

pub(crate) static mut POKEDEX: Option<Pokedex> = None;

pub fn set(dex: Pokedex) {
    unsafe { POKEDEX = Some(dex) }
}

pub fn pokedex_len() -> PokemonId {
	unsafe { POKEDEX.as_ref().map(|dex| dex.len()).unwrap_or_default() as PokemonId }
}