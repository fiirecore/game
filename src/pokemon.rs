use std::ffi::OsString;
use std::path::PathBuf;

use firecore_pokedex::pokemon::Pokemon;
use firecore_pokedex::pokemon::texture::PokemonTexture;
use macroquad::prelude::warn;

lazy_static::lazy_static! {
    pub static ref DEX_DIR: PathBuf = PathBuf::from("pokedex");
}

pub fn load() {	

	load_pokedex_v2();
	load_pokedex_v1();

	for path in crate::io::get_dir(DEX_DIR.join("moves")) {
		match crate::io::get_file_as_string(&path) {
			Ok(data) => {
				let result: Result<firecore_pokedex::moves::PokemonMove, toml::de::Error> = toml::from_str(&data);
				match result {
					Ok(pokemon_move) => {
						firecore_pokedex::MOVEDEX.insert(pokemon_move.number, pokemon_move);
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

fn load_pokedex_v2() {
	for directory in crate::io::get_dir(DEX_DIR.join("pokemon")) {
		for file in crate::io::get_dir(directory) {
			if let Some(ext) = file.extension() {
				if ext == OsString::from("toml") {
					match crate::io::get_file_as_string(&file) {
						Ok(data) => {
							let result: Result<firecore_pokedex::pokemon::Pokemon, toml::de::Error> = toml::from_str(&data);
							match result {
								Ok(mut pokemon) => {
									load_cry(&mut pokemon);
									firecore_pokedex::POKEDEX.insert(pokemon.data.number, pokemon);
								}
								Err(err) => {
									warn!("Could not read pokemon move at {:?} with error {}", &file, err);
								},
							}
						}
						Err(err) => {
							warn!("Could not read pokemon move at {:?} to string with error {}", &file, err);
						}
					}
				}
			}
		}
	}
}

#[deprecated(since = "0.2.5", note = "Use v2 function instead")]
fn load_pokedex_v1() {
	for path in crate::io::get_dir(DEX_DIR.join("entries")) {
		match crate::io::get_file_as_string(&path) {
			Ok(data) => {
				let result: Result<firecore_pokedex::pokemon::Pokemon, toml::de::Error> = toml::from_str(&data);
				match result {
					Ok(pokemon) => {
						firecore_pokedex::POKEDEX.insert(pokemon.data.number, pokemon);
					},
					Err(err) => {
						warn!("Could not read pokemon entry at {:?} with error {}", path, err);
					},
				}
			}
			Err(err) => {
				warn!("Could not read pokemon entry at {:?} to string with error {}", path, err);
			},
		}
	}
}

fn load_cry(pokemon: &mut Pokemon) {
	match crate::io::get_file(String::from("pokedex/pokemon/") + &pokemon.data.name + "/" + &pokemon.cry.path) {
		Some(bytes) => {
			frc_audio::kira::context::sound::SOUND_CONTEXT.add_sound(frc_audio::sound::Sound::Cry(pokemon.data.number), &*bytes);
		}
		None => {
			warn!("Could not get bytes of cry for pokemon {}", pokemon.data.name);
		}
	}	
	
}

pub fn pokemon_texture(name: &str, side: PokemonTexture) -> crate::util::graphics::Texture {
    let path = std::path::PathBuf::from(DEX_DIR.join("textures/normal")).join(side.path()).join(name.to_lowercase() + ".png");
    match crate::io::get_file(&path) {
        Some(file) => {
            return crate::util::graphics::texture::byte_texture(&file);
        }
        None => {
            macroquad::prelude::warn!("Could not find pokemon texture at {:?}", &path);
            return crate::util::graphics::texture::debug_texture();
        }
    }
}