use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use crate::tetra::{
    Context, 
    Result,
};
use storage::load;
use pokedex::{
    battle,
    serialize::SerializedDex,
};
use crate::audio::{
    sound::{Sound, add_sound},
    serialized::SerializedSoundData,
};
use crate::config::{Configuration, CONFIGURATION};
use pokedex::texture::{PokemonTextures, POKEMON_TEXTURES, ITEM_TEXTURES};
use deps::hash::HashMap;
use crate::graphics::byte_texture;

pub use firecore_text::init as text;

pub static LOADING_FINISHED: AtomicBool = AtomicBool::new(false);

pub fn seed_random(seed: u64) {
    deps::random::seed_global(seed);
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

    use deps::borrow::Identifiable;
    use ::pokedex::{
        types::PokemonType, 
        pokemon::{Pokemon, dex::{self as pokedex, Pokedex}, data::{PokedexData, Training, Breeding}},
        moves::dex as movedex,
        item::dex as itemdex,
    };

    let mut pokedex = Pokedex::with_capacity(dex.pokemon.len());

	let mut pokemon_textures = PokemonTextures::with_capacity(dex.pokemon.len());

    pokedex.insert(
        Pokemon::UNKNOWN, 
        Pokemon {
            id: Pokemon::UNKNOWN,
            name: "Unknown".to_string(),
            primary_type: PokemonType::default(),
            secondary_type: None,
            base: Default::default(),
            data: PokedexData {
                species: "Unknown".to_string(),
                height: 0,
                weight: 0,
            },
            training: Training {
                base_exp: 0,
                growth_rate: Default::default(),
            },
            breeding: Breeding {
                gender: None,
            },
            moves: Vec::new(),
        }
    );

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

    pokedex::set(pokedex);
    
	unsafe { POKEMON_TEXTURES = Some(pokemon_textures); }

	let mut movedex = HashMap::with_capacity(dex.moves.len());

    let mut battle_movedex = HashMap::new();

	for serialized_move in dex.moves {
        let pmove = serialized_move.pokemon_move;
        if let Some(battle_move) = serialized_move.battle_move {
            battle_movedex.insert(pmove.id, battle_move.into(ctx));
        }
		movedex.insert(pmove.id, pmove);
	}

    movedex::set(movedex);
    battle::dex::set(battle_movedex);

    let mut itemdex = HashMap::with_capacity(dex.items.len());

    let mut item_textures = HashMap::with_capacity(dex.items.len());

    for item in dex.items {
        item_textures.insert(item.item.id, byte_texture(ctx, &item.texture));
        itemdex.insert(item.item.id, item.item);
    }

    ::pokedex::texture::trainer::set_trainer_textures(dex.trainers.into_iter().map(|(k, bytes)| (k, byte_texture(ctx, &bytes))).collect());

    itemdex::set(itemdex);

    unsafe { ITEM_TEXTURES = Some(item_textures); }

    Ok(())

}

#[cfg(feature = "audio")]
pub fn audio(audio: crate::audio::serialized::SerializedAudio) {
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