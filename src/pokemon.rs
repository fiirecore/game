use macroquad::prelude::{info, warn};

use firecore_pokedex::{
	POKEDEX,
	MOVEDEX,
	serialized::SerializedDex,
};

use firecore_audio::{
	add_sound,
	SerializedSoundData,
	Sound
};

use crate::util::graphics::texture::byte_texture;
use crate::util::pokemon::PokemonTextures;

pub async fn load(textures: &mut PokemonTextures) {

	let dex: SerializedDex = bincode::deserialize(
		// &macroquad::prelude::load_file("assets/dex.bin").await.unwrap()
		include_bytes!("../assets/dex.bin")
	).unwrap();

	info!("Loading pokedex and moves!");

	firecore_pokedex::new();

	let pokedex = unsafe { POKEDEX.as_ref().unwrap() };

	for pokemon in dex.pokemon {
		
		textures.front.insert(pokemon.pokemon.data.id, byte_texture(&pokemon.front_png));
		textures.back.insert(pokemon.pokemon.data.id, byte_texture(&pokemon.back_png));
		textures.icon.insert(pokemon.pokemon.data.id, byte_texture(&pokemon.icon_png));

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
		
		pokedex.insert(pokemon.pokemon.data.id, pokemon.pokemon);
	}

	let movedex = unsafe { MOVEDEX.as_ref().unwrap() };

	for pokemon_move in dex.moves {
		movedex.insert(pokemon_move.id, pokemon_move);
	}

	info!("Finished loading pokedex and moves!");

}