use deps::hash::HashMap;
use firecore_audio_lib::sound::Sound;
use parking_lot::Mutex;
use parking_lot::const_mutex;

use crate::error::PlayAudioError;

pub static SOUND_MAP: Mutex<Option<HashMap<Sound, super::SoundHandle>>> = const_mutex(None);

pub fn play_sound(sound: &Sound) -> Result<(), PlayAudioError> {
    match SOUND_MAP.lock().as_ref() {
        Some(map) => match map.get(sound) {
            Some(handle) => {
                let channel = sdl2::mixer::Channel::all();
                match channel.play(&handle.0, 0) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(err.into()),
                }
            }
            None => {
                Err(PlayAudioError::Missing)
            }
        },
        None => Ok(()), //Err(PlayAudioError::Uninitialized),
    }
    
}