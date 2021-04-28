use util::hash::HashMap;
use firecore_audio_lib::sound::Sound;
use kira::sound::handle::SoundHandle;
use parking_lot::Mutex;
use parking_lot::const_mutex;

use crate::error::PlayAudioError;

pub static SOUND_MAP: Mutex<Option<HashMap<Sound, SoundHandle>>> = const_mutex(None);

pub fn play_sound(sound: &Sound) -> Result<(), PlayAudioError> {
    match SOUND_MAP.lock().as_mut() {
        Some(map) => match map.get_mut(sound) {
            Some(handle) => {
                match handle.play(kira::instance::InstanceSettings::default()) {
                    Ok(_) => {
                        Ok(())
                    }
                    Err(err) => {
                        Err(PlayAudioError::PlayError(err))
                    }
                }
            }
            None => {
                Err(PlayAudioError::Missing)
            }
        },
        None => Err(PlayAudioError::Uninitialized),
    }
    
}