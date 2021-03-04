use std::path::PathBuf;

use firecore_pokedex::pokemon::texture::PokemonTexture;
use macroquad::prelude::warn;

lazy_static::lazy_static! {
    pub static ref DEX_DIR: PathBuf = PathBuf::from("pokedex");
}

pub fn load() {

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