use deps::tetra::Context;
use firecore_audio_lib::{
    music::{MusicId, MusicName},
    serialized::SerializedMusicData,
};

use super::error::{AddAudioError, PlayAudioError};

#[cfg(feature = "audio")]
pub static MUSIC_ID_MAP: parking_lot::Mutex<Option<deps::hash::HashMap<MusicName, MusicId>>> = parking_lot::const_mutex(None);

pub fn add_track(music_data: SerializedMusicData) -> Result<(), AddAudioError> {
    #[cfg(feature = "audio")] {
        match MUSIC_ID_MAP.lock().as_mut() {
            Some(map) => {
                map.insert(music_data.music.name.clone(), music_data.music.track);
                super::backend::context::add_track(music_data)
            }
            None => {
                Err(AddAudioError::Uninitialized)
            }
        }
    }
    #[cfg(not(feature = "audio"))] {
        Ok(())
    }
}

pub fn get_music_id(name: &str) -> Option<Option<MusicId>> {
    #[cfg(feature = "audio")] {
        Some(match MUSIC_ID_MAP.lock().as_ref() {
            Some(map) => map.get(name).copied(),
            None => None,
        })
    }
    #[cfg(not(feature = "audio"))] {
        None
    }
}

pub fn play_music_id(ctx: &Context, id: MusicId) -> Result<(), PlayAudioError> {
    super::backend::music::play_music(ctx, id)
}

pub fn play_music_named(ctx: &Context, name: &str) -> Result<(), PlayAudioError> {
    #[cfg(feature = "audio")]
    match get_music_id(&name.to_string()) {
        Some(id) => match id {
            Some(id) => {
                play_music_id(ctx, id)?;
                Ok(())
            }
            None => {
                Err(PlayAudioError::Missing)
            }
        }
        None => Ok(()),
    }
    #[cfg(not(feature = "audio"))]
    Ok(())
}

pub fn get_current_music() -> Option<MusicId> {
    super::backend::music::get_current_music()
}