use firecore_audio_lib::serialized::SerializedMusicData;
use firecore_audio_lib::music::{MusicId, MusicName};

use crate::error::AddAudioError;
use crate::error::PlayAudioError;

#[cfg(feature = "play")]
pub static MUSIC_ID_MAP: parking_lot::Mutex<Option<deps::hash::HashMap<MusicName, MusicId>>> = parking_lot::const_mutex(None);

pub fn add_track(music_data: SerializedMusicData) -> Result<(), AddAudioError> {
    #[cfg(feature = "play")] {
        match MUSIC_ID_MAP.lock().as_mut() {
            Some(map) => {
                map.insert(music_data.music.name.clone(), music_data.music.track);
                #[cfg(all(not(any(target_arch = "wasm32", target_os = "android")), feature = "play"))]
                crate::backend::context::add_track(music_data)?;
                Ok(())
            }
            None => {
                Err(AddAudioError::Uninitialized)
            }
        }
    }
    #[cfg(not(feature = "play"))] {
        Ok(())
    }
}

pub fn get_music_id(name: &str) -> Option<Option<MusicId>> {
    #[cfg(feature = "play")] {
        Some(match MUSIC_ID_MAP.lock().as_ref() {
            Some(map) => map.get(name).map(|id| *id),
            None => None,
        })
    }
    #[cfg(not(feature = "play"))] {
        None
    }
}

pub fn play_music_id(id: MusicId) -> Result<(), PlayAudioError> {
    #[cfg(all(not(any(target_arch = "wasm32", target_os = "android")), feature = "play"))] {
        crate::backend::music::play_music(id)
    }
    #[cfg(any(target_arch = "wasm32", target_os = "android", not(feature = "play")))]
    Ok(())
}

pub fn play_music_named(name: &str) -> Result<(), PlayAudioError> {
    #[cfg(feature = "play")]
    match get_music_id(&name.to_string()) {
        Some(id) => match id {
            Some(id) => {
                play_music_id(id)?;
                Ok(())
            }
            None => {
                Err(PlayAudioError::Missing)
            }
        }
        None => Ok(()),
    }
    #[cfg(not(feature = "play"))]
    Ok(())
}

pub fn get_current_music() -> Option<MusicId> {
    #[cfg(feature = "play")] {
        crate::backend::music::get_current_music()
    }
}