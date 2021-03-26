use dashmap::DashMap;

use macroquad::prelude::{info, warn};

use firecore_pokedex::{
	POKEDEX,
	MOVEDEX,
	pokemon::{
		PokemonId,
		texture::PokemonTexture,
	}
};

use firecore_audio::{
	add_sound,
	SerializedSoundData,
	Sound
};

use crate::util::graphics::{Texture, texture::{byte_texture, debug_texture}};

lazy_static::lazy_static! {
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
		
		FRONT_TEXTURES.insert(pokemon.pokemon.data.id, byte_texture(&pokemon.front_png));
		BACK_TEXTURES.insert(pokemon.pokemon.data.id, byte_texture(&pokemon.back_png));
		ICON_TEXTURES.insert(pokemon.pokemon.data.id, byte_texture(&pokemon.icon_png));

		if !pokemon.cry_ogg.is_empty() {
			if let Err(err) = add_sound(
				SerializedSoundData {
					bytes: pokemon.cry_ogg,
					sound: Sound {
						name: String::from("Cry"),
						variant: pokemon.pokemon.data.id,
					}
				}
			) {
				warn!("Error adding pokemon cry: {}", err);
			}
		}
		
		POKEDEX.insert(pokemon.pokemon.data.id, pokemon.pokemon);
	}

	for pokemon_move in dex.moves {
		MOVEDEX.insert(pokemon_move.id, pokemon_move);
	}

	info!("Finished loading pokedex and moves!");

}

pub fn pokemon_texture(id: &PokemonId, side: PokemonTexture) -> Texture {
	match side {
	    PokemonTexture::Front => FRONT_TEXTURES.get(id).map(|tex| *tex),
	    PokemonTexture::Back => BACK_TEXTURES.get(id).map(|tex| *tex),
	    PokemonTexture::Icon => ICON_TEXTURES.get(id).map(|tex| *tex),
	}.unwrap_or(debug_texture())
}