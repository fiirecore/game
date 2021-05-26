use deps::hash::HashMap;
use deps::tetra::{Context, audio::{Sound, SoundInstance}};
use parking_lot::{Mutex, const_mutex};
use firecore_audio_lib::music::{MusicId, MusicData};

use crate::audio::error::{PlayAudioError, Lockable};

pub(crate) static MUSIC_MAP: Mutex<Option<HashMap<MusicId, (MusicData, Sound)>>> = const_mutex(None);
pub(crate) static CURRENT_MUSIC: Mutex<Option<(MusicId, SoundInstance)>> = const_mutex(None);

pub fn play_music(ctx: &Context, music: MusicId) -> Result<(), PlayAudioError> {
    match CURRENT_MUSIC.try_lock() {
        Some(mut current) => {
            if let Some((_, instance)) = current.take() {
                instance.stop();
            }
        }
        None => {
            return Err(PlayAudioError::LockError(Lockable::CurrentMusic));
        }
    }
    match MUSIC_MAP.lock().as_mut() {
        Some(map) => {
            match map.get_mut(&music) {
                Some((_music_data, audio)) => {
                    match CURRENT_MUSIC.try_lock() {
                        Some(mut current) => {
                            // let loop_start = music_data.loop_start.unwrap_or_default();
                            match audio.play(ctx) {
                                Ok(instance) => {
                                    instance.set_repeating(true);
                                    instance.set_volume(0.3);
                                    *current = Some((music, instance));
                                    Ok(())
                                }
                                Err(err) => {
                                    Err(PlayAudioError::TetraError(err))
                                }
                            }
                        }
                        None => {
                            Err(PlayAudioError::LockError(Lockable::CurrentMusic))
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
    CURRENT_MUSIC.lock().as_ref().map(|(id, _)| *id)
}