use serde::{Deserialize, Serialize};

pub mod effective;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
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