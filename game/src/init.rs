use data::{store, get, configuration::Configuration, player::PlayerSaves};
use pokedex::{
    POKEDEX,
    MOVEDEX,
    ITEMDEX,
    serialize::SerializedDex,
};
use audio::{
    add_sound,
    SerializedSoundData,
    Sound
};

pub use firecore_text::init as text;

use crate::graphics::byte_texture;
use crate::textures::{PokemonTextures, POKEMON_TEXTURES, ITEM_TEXTURES};
use util::hash::HashMap;

pub fn seed_randoms(seed: u64) {
    pokedex::pokemon::POKEMON_RANDOM.seed(seed);
}

pub async fn data() {
    store::<Configuration>().await;
    store::<PlayerSaves>().await;

    {

        let config = get::<Configuration>().expect("Could not get configuration!");

        input::keyboard::load(config.controls.clone());

        if config.touchscreen {
            input::touchscreen::touchscreen(true);
        }

    }

}

pub fn pokedex(dex: SerializedDex) {

	let mut textures = PokemonTextures::default();

	pokedex::new();

	let pokedex = unsafe { POKEDEX.as_mut().unwrap() };

	for pokemon in dex.pokemon {
		
		textures.front.insert(pokemon.pokemon.data.id, byte_texture(&pokemon.front_png));
		textures.back.insert(pokemon.pokemon.data.id, byte_texture(&pokemon.back_png));
		textures.icon.insert(pokemon.pokemon.data.id, byte_texture(&pokemon.icon_png));

		if !pokemon.cry_ogg.is_empty() {
			if let Err(_) = add_sound(
				SerializedSoundData {
					bytes: pokemon.cry_ogg,
					sound: Sound {
						name: String::from("Cry"),
						variant: Some(pokemon.pokemon.data.id),
					}
				}
			) {
				// warn!("Error adding pokemon cry: {}", err);
			}
		}
		
		pokedex.insert(pokemon.pokemon.data.id, pokemon.pokemon);
	}

	let movedex = unsafe { MOVEDEX.as_mut().unwrap() };

	for pokemon_move in dex.moves {
		movedex.insert(pokemon_move.id, pokemon_move);
	}

    let itemdex = unsafe { ITEMDEX.as_mut().unwrap() };

    let mut item_textures = HashMap::with_capacity(dex.items.len());

    for item in dex.items {
        item_textures.insert(item.item.id, byte_texture(&item.texture));
        itemdex.insert(item.item.id, item.item);
    }

    unsafe { ITEM_TEXTURES = Some(item_textures); }

	unsafe { POKEMON_TEXTURES = Some(textures); }

}

#[cfg(feature = "audio")]
pub fn audio(audio: audio::SerializedAudio) {
    use macroquad::prelude::error;

    if let Err(err) = audio::create() {
        error!("{}", err);
    } else {
        #[cfg(not(target = "wasm32"))] {
            std::thread::spawn( || {
                if let Err(err) = audio::load(audio) {
                    error!("Could not load audio files with error {}", err);
                }
            });
        }
    
        #[cfg(target = "wasm32")] {
            if let Err(err) = audio::load(audio) {
                error!("Could not load audio files with error {}", err);
            }
        }
    }    
}