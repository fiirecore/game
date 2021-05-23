use deps::hash::HashMap;
use firecore_audio_lib::music::{MusicId, MusicData};
use parking_lot::{Mutex, const_mutex};

use crate::error::PlayAudioError;

pub static MUSIC_MAP: Mutex<Option<HashMap<MusicId, (MusicData, super::MusicHandle)>>> = const_mutex(None);
pub static CURRENT_MUSIC: Mutex<Option<MusicId>> = const_mutex(None);

pub fn play_music(music: MusicId) -> Result<(), PlayAudioError> {
    match CURRENT_MUSIC.try_lock() {
        Some(mut current) => {
            *current = None;
        }
        None => {
            return Err(PlayAudioError::CurrentLocked);
        }
    }
    match MUSIC_MAP.lock().as_mut() {
        Some(map) => {
            match map.get_mut(&music) {
                Some((music_data, audio)) => {
                    match CURRENT_MUSIC.try_lock() {
                        Some(mut current) => {
                            let loop_start = music_data.loop_start.unwrap_or_default();
                            match audio.0.play(-1) {
                                Ok(()) => {
                                    *current = Some(music);
                                    Ok(())
                                }
                                Err(err) => Err(PlayAudioError::SdlError(err)),
                            }
                        }
                        None => {
                            Err(PlayAudioError::CurrentLocked)
                        }
                    }
                    
                }
                None => {
                    Err(PlayAudioError::Missing)
                }
            }
        }
        None => {
            // Err(PlayAudioError::Uninitialized)
            Ok(())
        }
    }
       
}

pub fn get_current_music() -> Option<MusicId> {
    *CURRENT_MUSIC.lock()
}