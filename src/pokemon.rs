use std::path::PathBuf;

use dashmap::DashMap;
use firecore_pokedex::PokemonId;
// use firecore_pokedex::pokemon::Pokemon;
use firecore_pokedex::pokemon::texture::PokemonTexture;
use macroquad::prelude::info;
// use macroquad::prelude::load_file;
// use macroquad::prelude::warn;

use crate::util::graphics::Texture;
use crate::util::graphics::texture::byte_texture;
use crate::util::graphics::texture::debug_texture;

lazy_static::lazy_static! {
    pub static ref DEX_DIR: PathBuf = PathBuf::from("pokedex");
	pub static ref FRONT_TEXTURES: DashMap<PokemonId, Texture> = DashMap::new();
	pub static ref BACK_TEXTURES: DashMap<PokemonId, Texture> = DashMap::new();
	pub static ref ICON_TEXTURES: DashMap<PokemonId, Texture> = DashMap::new();
}

pub async fn load() {

	let dex: firecore_pokedex::serialized::SerializedDex = bincode::deserialize(&macroquad::prelude::load_file("assets/dex.bin").await.unwrap()).unwrap();

	// load_pokedex_v2();

	info!("Loading pokedex and moves!");

	for pokemon in dex.pokemon {
		// load_textures(&pokemon).await;
		
		FRONT_TEXTURES.insert(pokemon.pokemon.data.number, byte_texture(&pokemon.front_png));
		BACK_TEXTURES.insert(pokemon.pokemon.data.number, byte_texture(&pokemon.back_png));
		
		firecore_pokedex::POKEDEX.insert(pokemon.pokemon.data.number, pokemon.pokemon);
	}

	for pokemon_move in dex.moves {
		firecore_pokedex::MOVEDEX.insert(pokemon_move.number, pokemon_move);
	}

	info!("Finished loading pokedex and moves!");

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

// async fn load_cry(pokemon: &Pokemon) {
// 	if let Some(cry_path) = pokemon.cry_file.as_ref() {
// 		match macroquad::prelude::load_file(&(String::from("pokedex/pokemon/") + &pokemon.data.name + "/" + cry_path)).await {
// 			Ok(bytes) => {
// 				firecore_audio::add_sound(firecore_util::sound::Sound::Cry(pokemon.data.number), &*bytes);
// 			}
// 			Err(err) => {
// 				warn!("Could not get bytes of cry for pokemon {} with error {}", pokemon.data.name, err);
// 			}
// 		}	
// 	}	
// }

// const SIDES: &[PokemonTexture] = &[PokemonTexture::Front, PokemonTexture::Back];

// pub async fn load_textures(pokemon: &Pokemon) {
// 	let base_path = String::from("assets/pokedex/textures/");
// 	let front_path = base_path.clone() + "normal/front/" + pokemon.data.name.to_ascii_lowercase().as_str() + ".png";
// 	let back_path = base_path.clone() + "normal/back/" + pokemon.data.name.to_ascii_lowercase().as_str() + ".png";
// 	let icon_path = base_path + "icon/" + pokemon.data.name.to_ascii_lowercase().as_str() + ".png";
// 	match load_file(&front_path).await {
// 		Ok(bytes) => {
// 			FRONT_TEXTURES.insert(pokemon.data.number, byte_texture(&bytes));
// 		}
// 		Err(err) => {
// 			warn!("Could not load front texture for {} with error {}", pokemon.data.name, err);
// 		}
// 	}
// 	match load_file(&back_path).await {
// 		Ok(bytes) => {
// 			BACK_TEXTURES.insert(pokemon.data.number, byte_texture(&bytes));
// 		}
// 		Err(err) => {
// 			warn!("Could not load back texture for {} with error {}", pokemon.data.name, err);
// 		}
// 	}
// 	#[cfg(not(target_arch = "wasm32"))]
// 	match load_file(&icon_path).await {
// 	    Ok(bytes) => {
// 			ICON_TEXTURES.insert(pokemon.data.number, byte_texture(&bytes));
// 		}
// 	    Err(_) => {}
// 	}
// }

pub fn pokemon_texture(id: &PokemonId, side: PokemonTexture) -> Texture {
	match side {
	    PokemonTexture::Front => match FRONT_TEXTURES.get(id) {
	        Some(texture) => {
				*texture
			}
	        None => {
				debug_texture()
			}
	    },
	    PokemonTexture::Back => match BACK_TEXTURES.get(id) {
	        Some(texture) => {
				*texture
			}
	        None => {
				debug_texture()
			}
	    },
	    PokemonTexture::Icon => match ICON_TEXTURES.get(id) {
	        Some(texture) => {
				*texture
			}
	        None => {
				debug_texture()
			}
	    },
	}
	// let name: &str = &name.to_ascii_lowercase();
    
    // match crate::util::file::noasync::read_noasync(&path) {
    //     Some(file) => {
    //         return crate::util::graphics::texture::byte_texture(&file);
    //     }
    //     None => {
    //         macroquad::prelude::warn!("Could not find pokemon texture at {:?}", &path);
    //         return crate::util::graphics::texture::debug_texture();
    //     }
    // }
}