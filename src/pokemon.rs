use std::path::PathBuf;

use dashmap::DashMap;
use firecore_pokedex::PokemonId;
use firecore_pokedex::pokemon::texture::PokemonTexture;
use macroquad::prelude::info;

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

	let dex: firecore_pokedex::serialized::SerializedDex = bincode::deserialize(
		// &macroquad::prelude::load_file("assets/dex.bin").await.unwrap()
		include_bytes!("../assets/dex.bin")
	).unwrap();

	info!("Loading pokedex and moves!");

	for pokemon in dex.pokemon {
		// load_textures(&pokemon).await;
		
		FRONT_TEXTURES.insert(pokemon.pokemon.data.number, byte_texture(&pokemon.front_png));
		BACK_TEXTURES.insert(pokemon.pokemon.data.number, byte_texture(&pokemon.back_png));
		// info!("Adding texture for {}", pokemon.pokemon.data.name);
		ICON_TEXTURES.insert(pokemon.pokemon.data.number, byte_texture(&pokemon.icon_png));
		
		firecore_pokedex::POKEDEX.insert(pokemon.pokemon.data.number, pokemon.pokemon);
	}

	for pokemon_move in dex.moves {
		firecore_pokedex::MOVEDEX.insert(pokemon_move.number, pokemon_move);
	}

	info!("Finished loading pokedex and moves!");

}

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
}