extern crate firecore_audio as audio;

#[cfg(feature = "play")]
extern crate firecore_dependencies as deps;

use error::AddAudioError;

pub mod music;
pub mod sound;

pub mod error;
pub mod backend;

pub use audio::serialized;

#[cfg(feature = "audio")]
pub fn create() -> Result<(), AddAudioError> {
    *music::MUSIC_ID_MAP.lock() = Some(deps::hash::HashMap::new());
    backend::context::create()
}

#[cfg(feature = "audio")]
pub fn load(data: serialized::SerializedAudio) -> Result<(), AddAudioError> {
    for music_data in data.music {
        music::add_music(music_data)?;
    }
    for sound_data in data.sounds {
        sound::add_sound(sound_data)?;
    }
    Ok(())
}