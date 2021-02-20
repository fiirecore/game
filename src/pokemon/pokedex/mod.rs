use std::path::PathBuf;

use ahash::AHashMap as HashMap;
use macroquad::prelude::warn;

use super::Pokemon;
use super::PokemonId;
use super::moves::MoveInstance;
use super::moves::PokemonMove;

pub mod texture;

pub struct Pokedex {
	
	pub pokemon_list: HashMap<PokemonId, Pokemon>,
	pub move_list: HashMap<u16, PokemonMove>,
	
}

pub static LENGTH: usize = 386;

lazy_static::lazy_static! {
	pub static ref DEX_DIR: PathBuf = PathBuf::from("pokedex");
}


impl Pokedex {
	
	pub fn new() -> Pokedex {

		let mut pokemon_list = HashMap::new();

		for path in crate::io::get_dir(DEX_DIR.join("entries")) {
			match crate::io::get_file_as_string(&path) {
				Ok(data) => match Pokemon::from_string(&data) {
					Ok(pokemon) => {
						pokemon_list.insert(pokemon.data.number, pokemon);
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

		let mut move_list = HashMap::new();

	
		for path in crate::io::get_dir(DEX_DIR.join("moves")) {
			match crate::io::get_file_as_string(&path) {
				Ok(data) => {
					match PokemonMove::from_string(&data) {
						Ok(pokemon_move) => {
							move_list.insert(pokemon_move.number, pokemon_move);
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
		
		Pokedex {
			
			pokemon_list: pokemon_list,
			move_list: move_list,
			
		}
		
	}

	pub fn pokemon_from_id(&self, id: PokemonId) -> &Pokemon {
		match self.pokemon_list.get(&id) {
			Some(pokemon) => pokemon,
			None => {
				let pokemon = self.pokemon_list.values().next().unwrap();
				warn!("Pokemon with id {} could not be found! Returning first found Pokemon value ({}).", id, &pokemon.data.name);
				return pokemon;
			}
		}
	}

	pub fn moves_from_level(&self, pokemon_id: PokemonId, level: u8) -> Vec<MoveInstance> {
		self.pokemon_from_id(pokemon_id).moves_from_level(&self, level)
	}
	
}