use std::path::PathBuf;

use dashmap::DashMap as HashMap;
use macroquad::prelude::warn;

use super::Pokemon;
use super::PokemonId;
use super::moves::PokemonMove;

pub mod texture;

lazy_static::lazy_static! {
	pub static ref POKEDEX: HashMap<PokemonId, Pokemon> = HashMap::new();
	pub static ref MOVEDEX: HashMap<u16, PokemonMove> = HashMap::new();
	pub static ref DEX_DIR: PathBuf = PathBuf::from("pokedex");
}

pub fn load() {

	for path in crate::io::get_dir(DEX_DIR.join("entries")) {
		match crate::io::get_file_as_string(&path) {
			Ok(data) => match Pokemon::from_string(&data) {
				Ok(pokemon) => {
					POKEDEX.insert(pokemon.data.number, pokemon);
				},
				Err(err) => {
					warn!("Could not read pokemon entry at {:?} with error {}", path, err);
				},
			}
			Err(err) => {
				warn!("Could not read pokemon entry at {:?} to string with error {}", path, err);
			},
		}
	}

	for path in crate::io::get_dir(DEX_DIR.join("moves")) {
		match crate::io::get_file_as_string(&path) {
			Ok(data) => {
				match PokemonMove::from_string(&data) {
					Ok(pokemon_move) => {
						MOVEDEX.insert(pokemon_move.number, pokemon_move);
					}
					Err(err) => {
						warn!("Could not read pokemon move at {:?} with error {}", &path, err);
					},
				}
			}
			Err(err) => {
				warn!("Could not read pokemon move at {:?} to string with error {}", &path, err);
			}
		}
	}

}