use ahash::AHashMap;
use ahash::AHashSet;
use parking_lot::Mutex;
use zip::ZipArchive as Archive;
use std::io::Cursor;
use macroquad::prelude::warn;

use crate::io::data::pokemon::Pokemon;
use crate::io::data::pokemon::moves::pokemon_move::PokemonMove;
use crate::util::texture::debug_texture;

pub struct Pokedex {
	
	pub pokemon_list: AHashMap<usize, Pokemon>,
	pub move_list: AHashMap<String, PokemonMove>,
	
}

pub static LENGTH: usize = 386;
pub static DEX_DIR: &str = "pokedex/";

lazy_static::lazy_static! {
	static ref POKEDEX_ARCHIVE: Mutex<Archive<Cursor<&'static [u8; 1103274]>>> = Mutex::new(Archive::new(Cursor::new(include_bytes!("../../../include/pokedex.zip"))).expect("Could not read pokedex archive file in executable!"));
}

impl Pokedex {
	
	pub fn new() -> Pokedex {

		let mut archive = POKEDEX_ARCHIVE.lock();

		let mut pokemon_files: AHashSet<String> = AHashSet::new();
		let mut move_files: AHashSet<String> = AHashSet::new();
		
		for file in archive.file_names() {
			if file.starts_with('e') {
				pokemon_files.insert(file.to_string());
			} else if file.starts_with('m') {
				move_files.insert(file.to_string());
			} else if !file.starts_with('t') {
				warn!("Unknown file in pokedex zip: {}", file);
			}
		}

		let mut pokemon_list = AHashMap::new();

		for filename in pokemon_files {
			match archive.by_name(&filename) {
			    Ok(mut entry) => {
					let mut content = String::new();
					match std::io::Read::read_to_string(&mut entry, &mut content) {
					    Ok(_) => {
							match Pokemon::from_string(&content) {
								Ok(pokemon) => {
									pokemon_list.insert(pokemon.data.number, pokemon);
								},
								Err(err) => warn!("Could not read pokemon at {} with error {}", &filename, err),
							}
						}
					    Err(err) => warn!("Could not read pokemon at {} to string with error {}", &filename, err),
					}
				}
			    Err(err) => warn!("Could not get zipped file at {} with error {}", &filename, err),
			}
		}

		let mut move_list = AHashMap::new();

		for filename in move_files {
			match archive.by_name(&filename) {
			    Ok(mut entry) => {
					let mut content = String::new();
					match std::io::Read::read_to_string(&mut entry, &mut content) {
					    Ok(_) => {
							match PokemonMove::from_string(&content) {
								Ok(pokemon_move) => {
									move_list.insert(pokemon_move.name.clone(), pokemon_move);
								}
								Err(err) => warn!("Could not read pokemon move at {} with error {}", &filename, err)
							}
						}
					    Err(err) => warn!("Could not read pokemon move at {} to string with error {}", &filename, err),
					}
				}
			    Err(err) => warn!("Could not get zipped file at {} with error {}", &filename, err),
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

	pub fn moves_from_level(&self, pokemon_id: usize, level: u8) -> Vec<PokemonMove> {
		let mut moves = Vec::new();
		let pokemon = self.pokemon_from_id(pokemon_id);
		for learnable_move in &pokemon.moves {
			if learnable_move.level <= level {
				match self.move_list.get(&learnable_move.move_id) {
					Some(pokemon_move) => {
						moves.push(pokemon_move.clone());
					}
					None => {
						warn!("Could not add pokemon move {} to {}", &learnable_move.move_id, &pokemon.data.name)
					}
				}
			}
		}
		while moves.len() > 4 {
			moves.remove(0);
		}
		return moves;
	}

	pub fn pokemon_texture(pathend: String) -> crate::util::texture::Texture {
		let mut path = String::from("textures/normal/");
		path.push_str(&pathend);
		let mut bytes = Vec::new();
		match POKEDEX_ARCHIVE.lock().by_name(&path) {
		    Ok(mut zipfile) => {
				match std::io::Read::read_to_end(&mut zipfile, &mut bytes) {
				    Ok(_) => crate::util::texture::byte_texture(bytes.as_slice()),
				    Err(err) => {
						warn!("Could not read pokemon texture at {} with error {}", &path, err);
						debug_texture()
					}
				}
			}
		    Err(err) => {
				warn!("Could not find pokemon texture at {} with error {}", &path, err);
				debug_texture()
			}
		}
	}
	
}