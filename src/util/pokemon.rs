use ahash::AHashMap as HashMap;

use macroquad::prelude::{Texture2D, info, warn};

use firecore_pokedex::{
	POKEDEX,
	MOVEDEX,
	serialized::SerializedDex,
    pokemon::{PokemonId, texture::PokemonTexture}
};

use firecore_audio::{
	add_sound,
	SerializedSoundData,
	Sound
};

use crate::util::graphics::byte_texture;

pub async fn load(textures: &mut PokemonTextures) {

	let dex: SerializedDex = bincode::deserialize(
		// &macroquad::prelude::load_file("assets/dex.bin").await.unwrap()
		include_bytes!("../../assets/dex.bin")
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

use super::graphics::debug_texture;

#[derive(Default)]
pub struct PokemonTextures {

    pub front: HashMap<PokemonId, Texture2D>,
    pub back: HashMap<PokemonId, Texture2D>,
    pub icon: HashMap<PokemonId, Texture2D>,

}

impl PokemonTextures {

    pub fn pokemon_texture(&self, id: &PokemonId, side: PokemonTexture) -> Texture2D {
        match side {
            PokemonTexture::Front => self.front.get(id).map(|tex| *tex),
            PokemonTexture::Back => self.back.get(id).map(|tex| *tex),
            PokemonTexture::Icon => self.icon.get(id).map(|tex| *tex),
        }.unwrap_or(debug_texture())
    }

}