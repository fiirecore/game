#[cfg(feature = "play")]
extern crate firecore_dependencies as deps;

use error::AddAudioError;

mod music;
mod sound;

pub mod error;
pub mod backend;

pub use firecore_audio_lib::music::*;
pub use firecore_audio_lib::sound::*;

pub use firecore_audio_lib::serialized::*;

pub use music::{add_track, get_music_id, play_music_id, play_music_named, get_current_music};
pub use sound::{add_sound, play_sound};

#[cfg(feature = "audio")]
pub fn create() -> Result<(), AddAudioError> {
    *music::MUSIC_ID_MAP.lock() = Some(deps::hash::HashMap::new());
    backend::context::create()
}

#[cfg(feature = "audio")]
pub fn load(data: SerializedAudio) -> Result<(), AddAudioError> {
    for music_data in data.music {
        add_track(music_data)?;
    }
    for sound_data in data.sounds {
        add_sound(sound_data)?;
    }
    Ok(())
}