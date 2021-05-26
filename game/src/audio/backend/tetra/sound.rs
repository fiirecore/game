use deps::hash::HashMap;
use deps::tetra::Context;
use firecore_audio_lib::sound::Sound;
use parking_lot::Mutex;
use parking_lot::const_mutex;

use crate::audio::error::PlayAudioError;

pub(crate) static SOUND_MAP: Mutex<Option<HashMap<Sound, deps::tetra::audio::Sound>>> = const_mutex(None);

pub fn play_sound(ctx: &Context, sound: &Sound) -> Result<(), PlayAudioError> {
    match SOUND_MAP.lock().as_mut() {
        Some(map) => match map.get(sound) {
            Some(handle) => {
                match handle.play(ctx) {
                    Ok(instance) => {
                        instance.set_volume(0.3);
                        Ok(())
                    }
                    Err(err) => {
                        Err(PlayAudioError::TetraError(err))
                    }
                }
            }
            None => {
                Err(PlayAudioError::Missing)
            }
        },
        None => Ok(()), //Err(PlayAudioError::Uninitialized),
    }
    
}