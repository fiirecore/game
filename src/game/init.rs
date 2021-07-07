use super::{
    config::{Configuration, CONFIGURATION},
    storage::{try_load, save_locally},
};
pub use engine::graphics::text::init as text;
use engine::{
    audio::{
        serialized::SerializedSoundData,
        sound::{add_sound, Sound},
    },
    input,
    tetra::{Context, Result},
};
use pokedex::{
    serialize::{SerializedDex, SerializedPokemon},
    CRY_ID,
};
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

pub static LOADING_FINISHED: AtomicBool = AtomicBool::new(false);

pub fn seed_random(seed: u64) {
    deps::random::seed_global(seed);
}

pub fn logger() {
    use log::LevelFilter;
    use simple_logger::SimpleLogger;

    // Initialize logger

    let logger = SimpleLogger::new();

    #[cfg(debug_assertions)]
    let logger = logger.with_level(LevelFilter::Trace);
    #[cfg(not(debug_assertions))]
    let logger = logger.with_level(LevelFilter::Info);

    logger
        .init()
        .unwrap_or_else(|err| panic!("Could not initialize logger with error {}", err));
}

pub fn configuration() -> Result {
    let config = try_load::<Configuration>(save_locally())
        .unwrap_or_else(|err| panic!("Could not read configuration with error {}", err));
    // store::<PlayerSaves>().await;

    {
        input::keyboard::load(config.controls.clone());
        input::controller::load(input::controller::default_button_map());

        // if config.touchscreen {
        //     crate::input::touchscreen::touchscreen(true);
        // }
    }

    unsafe { CONFIGURATION = Some(config) };

    Ok(())
}

pub fn pokedex(ctx: &mut Context, dex: SerializedDex) -> Result {
    pokedex::init(
        ctx,
        dex,
        #[cfg(feature = "audio")]
        |pokemon: &mut SerializedPokemon| {
            if !pokemon.cry_ogg.is_empty() {
                if let Err(_) = add_sound(SerializedSoundData {
                    bytes: std::mem::take(&mut pokemon.cry_ogg),
                    sound: Sound::variant(CRY_ID, Some(pokemon.pokemon.id)),
                }) {
                    // warn!("Error adding pokemon cry: {}", err);
                }
            }
        }
    )
}

#[cfg(feature = "audio")]
use {engine::audio, log::error};

#[cfg(feature = "audio")]
pub fn audio(audio: audio::serialized::SerializedAudio) {
    if let Err(err) = audio::create() {
        error!("{}", err);
    } else {
        std::thread::spawn(|| {
            if let Err(err) = audio::load(audio) {
                error!("Could not load audio files with error {}", err);
            }
        });
    }
}

pub fn finished_loading() {
    LOADING_FINISHED.store(true, Relaxed);
}
