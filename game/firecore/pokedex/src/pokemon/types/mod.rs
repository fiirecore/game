use serde::{Deserialize, Serialize};

mod effective;
pub use effective::*;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)] // To - do: move module
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