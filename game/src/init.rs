use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use crate::tetra::{
    Context, 
    Result,
};
use storage::load;
use pokedex::{
    pokemon::POKEDEX,
    moves::{MOVEDEX, GAME_MOVE_DEX},
    item::ITEMDEX,
    serialize::SerializedDex,
};
use crate::audio::{
    add_sound,
    SerializedSoundData,
    Sound,
};
use crate::config::{Configuration, CONFIGURATION};
use pokedex::texture::{PokemonTextures, POKEMON_TEXTURES, ITEM_TEXTURES};
use deps::hash::HashMap;

pub use firecore_text::init as text;

pub static LOADING_FINISHED: AtomicBool = AtomicBool::new(false);

pub fn seed_randoms(seed: u64) {
    pokedex::pokemon::POKEMON_RANDOM.seed(seed);
    #[cfg(feature = "world")]
    crate::world::seed_randoms(seed);
    #[cfg(feature = "battle")]
	crate::battle::BATTLE_RANDOM.seed(seed);
}

pub fn configuration() -> Result {
    let config = load::<Configuration>();
    // store::<PlayerSaves>().await;

    {

        crate::input::keyboard::load(config.controls.clone());
        crate::input::controller::load(crate::input::controller::default_button_map());

        // if config.touchscreen {
        //     crate::input::touchscreen::touchscreen(true);
        // }

    }

    unsafe { CONFIGURATION = Some(config) };

    Ok(())

}

pub fn pokedex(ctx: &mut Context, dex: SerializedDex) -> Result {

    let pokedex = unsafe {
        POKEDEX.get_or_insert(HashMap::with_capacity(dex.pokemon.len()))
    };

	let mut pokemon_textures = PokemonTextures::with_capacity(dex.pokemon.len());

	for pokemon in dex.pokemon {

        pokemon_textures.insert(ctx, &pokemon)?;

        #[cfg(feature = "audio")]
		if !pokemon.cry_ogg.is_empty() {
			if let Err(_) = add_sound(
				SerializedSoundData {
					bytes: pokemon.cry_ogg,
					sound: Sound::variant(crate::CRY_ID, Some(pokemon.pokemon.id)),
				}
			) {
				// warn!("Error adding pokemon cry: {}", err);
			}
		}
		
		pokedex.insert(pokemon.pokemon.id, pokemon.pokemon);
	}
    
	unsafe { POKEMON_TEXTURES = Some(pokemon_textures); }

	let movedex = unsafe {
        MOVEDEX.get_or_insert(HashMap::with_capacity(dex.moves.len()))
    };

    let game_movedex = unsafe {
        GAME_MOVE_DEX.get_or_insert(HashMap::new())
    };

	for serialized_move in dex.moves {
        let pmove = serialized_move.pokemon_move;
        if let Some(game_move) = serialized_move.game_move {
            game_movedex.insert(pmove.id, game_move);
        }
        // if let Some(script) = pmove.battle_script.as_mut() {
        //     if !pokemon_move.battle_script_texture.is_empty() {
        //         script.texture = Some(byte_texture(&pokemon_move.battle_script_texture));
        //     }
        // }
		movedex.insert(pmove.id, pmove);
	}

    let itemdex = unsafe {
        ITEMDEX.get_or_insert(HashMap::with_capacity(dex.items.len()))
    };

    let mut item_textures = HashMap::with_capacity(dex.items.len());

    for item in dex.items {
        item_textures.insert(item.item.id, crate::graphics::byte_texture(ctx, &item.texture));
        itemdex.insert(item.item.id, item.item);
    }

    unsafe { ITEM_TEXTURES = Some(item_textures); }

    Ok(())

}

#[cfg(feature = "audio")]
pub fn audio(audio: crate::audio::SerializedAudio) {
    use crate::log::error;    

    if let Err(err) = crate::audio::create() {
        error!("{}", err);
    } else {
        std::thread::spawn( || {
            if let Err(err) = crate::audio::load(audio) {
                error!("Could not load audio files with error {}", err);
            }
        });
    }    
}

pub fn finished_loading() {
    LOADING_FINISHED.store(true, Relaxed);
}