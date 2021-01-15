use std::collections::HashMap;
use std::path::PathBuf;

use log::error;
use log::warn;

use crate::io::data::pokemon::moves::pokemon_move::PokemonMove;
use crate::io::data::pokemon::pokemon::Pokemon;
use crate::util::file_util::asset_as_pathbuf;

pub struct Pokedex {
	
	pub pokemon_list: HashMap<usize, Pokemon>,
	pub move_list: HashMap<String, PokemonMove>,
	
}

pub static LENGTH: usize = 386;
pub static DEX_DIR: &str = "pokedex/";

impl Pokedex {
	
	pub fn new() -> Pokedex {
		
		Pokedex {
			
			pokemon_list: HashMap::new(),
			move_list: HashMap::new(),
			
		}
		
	}

	pub fn pokemon_from_id(&self, id: usize) -> &Pokemon {
			return self.pokemon_list.get(&id).unwrap_or(self.pokemon_list.get(&1).unwrap());
	}

	pub fn moves_from_level(&self, pokemon_id: usize, level: u8) -> Vec<PokemonMove> {
		let mut moves = Vec::new();
		let pokemon = self.pokemon_from_id(pokemon_id);
		for index in 0..level+1 {
			if let Some(pkmn_move_str) = pokemon.learnable_moves.get(&index) {
				for move_name in pkmn_move_str {
					match self.move_list.get(move_name) {
						Some(pokemon_move) => {
							moves.push(pokemon_move.clone());
						}
						None => {
							warn!("Could not add pokemon move {} to {}", move_name, &pokemon.data.name)
						}
					}
				}								
			}
		}
		while moves.len() > 4 {
			moves.remove(0);
		}
		return moves;
	}


	
	
	pub fn load(&mut self) {
		
		let entry_path = PathBuf::from(DEX_DIR).join("entries");

		match std::fs::read_dir(asset_as_pathbuf(&entry_path)) {
			Ok(dir) => {
				for entry in dir.map(|res| res.map(|e| e.path())) {
					match entry {
						Ok(path) => {
							if let Some(pokemon_entry) = Pokemon::new(path) {
								self.pokemon_list.insert(pokemon_entry.data.number, pokemon_entry);
							}					
						}
						Err(e) => {
							warn!("Error fetching pokemon entry at {:?} with error: {}", entry_path, e);
						}
					}
				}
			},
			Err(err) => {
				error!("Problem fetching pokemon entry directory with error: {}", err);
			},
		}

		let move_path = PathBuf::from(DEX_DIR).join("moves");

		match std::fs::read_dir(asset_as_pathbuf(&move_path)) {
			Ok(dir) => {
				for entry in dir.map(|res| res.map(|e| e.path())) {
					match entry {
						Ok(path) => {
							let move_entry = PokemonMove::load_move(path);
							if let Some(pkmn_move) = move_entry {
								self.move_list.insert(pkmn_move.name.clone(), pkmn_move);
							}
						}
						Err(e) => {
							warn!("Error fetching move toml at {:?} with error: {}", move_path, e);
						}
					}
				}
			}
			Err(err) => {
				error!("Problem fetching moves directory with error: {}", err);
			}
		}
		
	}
	
}