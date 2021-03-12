use std::ffi::OsString;
use std::io::Read;
use std::path::PathBuf;

use firecore_pokedex::moves::PokemonMove;
use firecore_pokedex::pokemon::Pokemon;
use firecore_pokedex::pokemon::texture::PokemonTexture;
use macroquad::prelude::warn;

lazy_static::lazy_static! {
    pub static ref DEX_DIR: PathBuf = PathBuf::from("pokedex");
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct DexSerialized {

	pub pokemon: Vec<Pokemon>,
	pub moves: Vec<PokemonMove>,

}

pub async fn load() {

	let dex: DexSerialized = bincode::deserialize(&macroquad::prelude::load_file("assets/dex.bin").await.unwrap()).unwrap();

	// load_pokedex_v2();

	for pokemon in dex.pokemon {
		firecore_pokedex::POKEDEX.insert(pokemon.data.number, pokemon);
	}

	for pokemon_move in dex.moves {
		firecore_pokedex::MOVEDEX.insert(pokemon_move.number, pokemon_move);
	}

}

// fn load_pokedex_v2() {
// 	for directory in crate::io::get_dir(DEX_DIR.join("pokemon")) {
// 		for file in crate::io::get_dir(directory) {
// 			if let Some(ext) = file.extension() {
// 				if ext == OsString::from("toml") {
// 					match crate::io::get_file_as_string(&file) {
// 						Ok(data) => {
// 							let result: Result<firecore_pokedex::pokemon::Pokemon, toml::de::Error> = toml::from_str(&data);
// 							match result {
// 								Ok(pokemon) => {
// 									load_cry(&pokemon);
// 									firecore_pokedex::POKEDEX.insert(pokemon.data.number, pokemon);
// 								}
// 								Err(err) => {
// 									warn!("Could not read pokemon move at {:?} with error {}", &file, err);
// 								},
// 							}
// 						}
// 						Err(err) => {
// 							warn!("Could not read pokemon move at {:?} to string with error {}", &file, err);
// 						}
// 					}
// 				}
// 			}
// 		}
// 	}
// }

async fn load_cry(pokemon: &Pokemon) {
	if let Some(cry_path) = pokemon.cry_file.as_ref() {
		match macroquad::prelude::load_file(&(String::from("pokedex/pokemon/") + &pokemon.data.name + "/" + cry_path)).await {
			Ok(bytes) => {
				firecore_audio::add_sound(firecore_audio::sound::Sound::Cry(pokemon.data.number), &*bytes);
			}
			Err(err) => {
				warn!("Could not get bytes of cry for pokemon {} with error {}", pokemon.data.name, err);
			}
		}	
	}	
}

pub fn pokemon_texture(name: &str, side: PokemonTexture) -> crate::util::graphics::Texture {
	let name: &str = &name.to_ascii_lowercase();
    let path = String::from("assets/pokedex/textures/normal/") + side.path() + "/" + name + ".png";
    match crate::util::file::noasync::read_noasync(&path) {
        Some(file) => {
            return crate::util::graphics::texture::byte_texture(&file);
        }
        None => {
            macroquad::prelude::warn!("Could not find pokemon texture at {:?}", &path);
            return crate::util::graphics::texture::debug_texture();
        }
    }
}