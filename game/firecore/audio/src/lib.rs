#[cfg(feature = "play")]
extern crate firecore_util as util;

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

#[cfg(feature = "play")]
pub fn create() -> Result<(), AddAudioError> {
    *music::MUSIC_ID_MAP.lock() = Some(util::hash::HashMap::new());
    #[cfg(not(target_arch = "wasm32"))] {
        backend::kira::context::create().map_err(|err| AddAudioError::SetupError(err))
    }
    #[cfg(target_arch = "wasm32")]
    Ok(())
}

#[cfg(feature = "play")]
pub fn load(data: SerializedAudio) -> Result<(), AddAudioError> {
    for music_data in data.music {
        add_track(music_data)?;
    }
    for sound_data in data.sounds {
        add_sound(sound_data)?;
    }
    Ok(())
}