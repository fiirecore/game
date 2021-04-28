use firecore_audio_lib::serialized::SerializedMusicData;
use firecore_audio_lib::music::{MusicId, MusicName};

use crate::error::AddAudioError;
use crate::error::PlayAudioError;

#[cfg(feature = "play")]
pub static MUSIC_ID_MAP: parking_lot::Mutex<Option<util::hash::HashMap<MusicName, MusicId>>> = parking_lot::const_mutex(None);

pub fn add_track(music_data: SerializedMusicData) -> Result<(), AddAudioError> {
    #[cfg(feature = "play")] {
        match MUSIC_ID_MAP.lock().as_mut() {
            Some(map) => {
                map.insert(music_data.music.name.clone(), music_data.music.track);
                #[cfg(all(not(target_arch = "wasm32"), feature = "kira"))]
                crate::backend::kira::context::add_track(music_data)?;
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

pub fn get_music_id(name: &str) -> Result<Option<MusicId>, PlayAudioError> {
    #[cfg(feature = "play")] {
        match MUSIC_ID_MAP.lock().as_ref() {
            Some(map) => Ok(map.get(name).map(|id| *id)),
            None => Err(PlayAudioError::Uninitialized),
        }
    }
    #[cfg(not(feature = "play"))] {
        Ok(None)
    }
}

pub fn play_music_id(id: MusicId) -> Result<(), PlayAudioError> {
    #[cfg(all(not(target_arch = "wasm32"), feature = "play"))]
    crate::backend::kira::music::play_music(id)?;
    Ok(())
}

pub fn play_music_named(name: &str) -> Result<(), PlayAudioError> {
    #[cfg(feature = "play")]
    match get_music_id(&name.to_string()) {
        Ok(id) => match id {
            Some(id) => {
                play_music_id(id)?;
                Ok(())
            }
            None => {
                Err(PlayAudioError::Missing)
            }
        }
        Err(err) => {
            Err(err)
        }
    }
    #[cfg(not(feature = "play"))]
    Ok(())
}

pub fn get_current_music() -> Option<MusicId> {
    #[cfg(all(not(target_arch = "wasm32"), feature = "play"))] {
        crate::backend::kira::music::get_current_music()
    }
    #[cfg(not(feature = "play"))] {
        None
    }
}