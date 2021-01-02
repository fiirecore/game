use std::{collections::HashMap, ffi::OsString};
use std::path::PathBuf;

use crate::{game::pokedex::pokemon::pokemon::Pokemon, util::file_util::UNKNOWN_FILENAME_ERR};

use crate::util::file_util::asset_as_pathbuf;

use super::pokemon_move::{move_toml, pokemon_move::PokemonMove};

pub struct Pokedex {
	
	pub pokemon_list: HashMap<usize, Pokemon>,
	pub move_list: HashMap<String, PokemonMove>,
	
}

pub static DEX_DIR: &str = "pokedex/";

impl Pokedex {
	
	pub fn new() -> Pokedex {
		
		Pokedex {
			
			pokemon_list: HashMap::new(),
			move_list: HashMap::new(),
			
		}
		
	}
	
	pub fn load(&mut self) {
		
//		let paths: Vec<_> = read_dir(asset_as_pathbuf(ENTRY_DIR)).unwrap().map(|r| r.unwrap().path()).collect();

/*

		for x in 1..26 {
			let mut path = PathBuf::from(DEX_DIR);
			path.push("entries");
			let mut string = x.to_string();
			string.push_str(".toml");
			path.push(string);
			let mut p = Pokemon::new(asset_as_pathbuf(&path));
			p.load();
			self.pokemon_list.push(p)
		}

		*/

		let mut entrydir = PathBuf::from(DEX_DIR);
		entrydir.push("entries");

		let entries = std::fs::read_dir(asset_as_pathbuf(&entrydir)).unwrap();
		let entries = entries.map(|res| res.map(|e| e.path()));

		for entry in entries {
			match entry {
				Ok(path) => {
					if let Some(pokemon_entry) = Pokemon::new(path) {
						self.pokemon_list.insert(pokemon_entry.number, pokemon_entry);
					}					
				}
				Err(e) => {
					println!("Error fetching pokemon entry at {:?} with error: {}", entrydir.file_name().unwrap_or(&OsString::from(UNKNOWN_FILENAME_ERR)), e);
				}
			}
		}

		let mut movedir = PathBuf::from(DEX_DIR);
		movedir.push("moves");

		let entries = std::fs::read_dir(asset_as_pathbuf(&movedir))
                        .unwrap()
                        .map(|res| res.map(|e| e.path()));

			for entry in entries {
				match entry {
					Ok(path) => {
						let move_entry = move_toml::load_move_from_toml(path);
						if let Some(pkmn_move) = move_entry {
							self.move_list.insert(pkmn_move.name.clone(), pkmn_move);
						}
					}
					Err(e) => {
						println!("Error fetching move toml at {:?} with error: {}", movedir, e);
					}
				}
			}
		
		//for path in paths {
		//	let mut p = Pokemon::new(path);
		//	p.load();
		//	self.list.push(p);
		//}
		
	}

	pub fn pokemon_from_id(&self, id: usize) -> &Pokemon {
			return self.pokemon_list.get(&id).unwrap_or(self.pokemon_list.get(&1).unwrap());
	}
	
}