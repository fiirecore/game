use std::path::PathBuf;

use ahash::AHashMap;
use macroquad::prelude::warn;

use super::Pokemon;
use super::moves::MoveInstance;
use super::moves::PokemonMove;

pub mod texture;

pub struct Pokedex {
	
	pub pokemon_list: AHashMap<usize, Pokemon>,
	pub move_list: AHashMap<String, PokemonMove>,
	
}

pub static LENGTH: usize = 386;

lazy_static::lazy_static! {
	pub static ref DEX_DIR: PathBuf = PathBuf::from("pokedex");
}


impl Pokedex {
	
	pub fn new() -> Pokedex {

		let mut pokemon_list = AHashMap::new();

		let entries_dir = crate::io::ASSET_DIR.get_dir(DEX_DIR.join("entries")).expect("Could not get pokemon entries directory!");

		for file in entries_dir.files() {
			match file.contents_utf8() {
			    Some(data) => {
					match Pokemon::from_string(data) {
					    Ok(pokemon) => {
							pokemon_list.insert(pokemon.data.number, pokemon);
						},
						Err(err) => warn!("Could not read pokemon at {} with error {}", &file.path, err),
					}
				}
			    None => {
					warn!("Could not read pokemon at {} to string with", &file.path);
				}
			}
		}

		let mut move_list = AHashMap::new();

		let moves_dir = crate::io::ASSET_DIR.get_dir(DEX_DIR.join("moves")).expect("Could not get moves directory!");

		for file in moves_dir.files() {
			match file.contents_utf8() {
			    Some(data) => {
					match PokemonMove::from_string(data) {
						Ok(pokemon_move) => {
							move_list.insert(pokemon_move.name.clone(), pokemon_move);
						}
						Err(err) => warn!("Could not read pokemon move at {} with error {}", &file.path, err),
					}
				}
			    None => {
					warn!("Could not read pokemon move at {} to string", &file.path);
				}
			}
		}
		
		Pokedex {
			
			pokemon_list: pokemon_list,
			move_list: move_list,
			
		}
		
	}

	pub fn pokemon_from_id(&self, id: usize) -> &Pokemon {
		return self.pokemon_list.get(&id).unwrap_or(self.pokemon_list.get(&1).unwrap());
	}

	pub fn moves_from_level(&self, pokemon_id: usize, level: u8) -> Vec<MoveInstance> {
		self.pokemon_from_id(pokemon_id).moves_from_level(&self, level)
	}
	
}