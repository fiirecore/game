use deps::hash::HashMap;
use firecore_audio_lib::music::MusicId;
use parking_lot::Mutex;
use parking_lot::const_mutex;
use soloud::Handle;

use crate::error::Lockable;
use crate::error::PlayAudioError;

use super::ReadableStream;

pub(crate) static MUSIC: Mutex<Option<HashMap<MusicId, ReadableStream>>> = const_mutex(None);
pub(crate) static CURRENT: Mutex<Option<(MusicId, Handle)>> = const_mutex(None);

pub fn play_music(music: MusicId) -> Result<(), PlayAudioError> {
    match super::context::CONTEXT.try_lock() {
        Some(mut context) => match context.as_mut() {
            Some(context) => match CURRENT.try_lock() {
                Some(mut current) => {
                    if let Some((_, handle)) = current.take() {
                        context.stop(handle);
                    }
                    match MUSIC.try_lock() {
                        Some(map) => match map.as_ref() {
                            Some(map) => match map.get(&music) {
                                Some(stream) => {
                                    let handle = context.play(&stream.0);
                                    *current = Some((music, handle));
                                    Ok(())
                                }
                                None => Err(PlayAudioError::Missing),
                            }
                            None => Err(PlayAudioError::Uninitialized),
                        },
                        None => Err(PlayAudioError::LockError(Lockable::AudioMap)),
                    }
                }
                None => Err(PlayAudioError::LockError(Lockable::CurrentMusic)),
            }
            None => Err(PlayAudioError::Uninitialized),
        },
        None => Err(PlayAudioError::LockError(Lockable::AudioManager)),
    }
}

pub fn get_current_music() -> Option<MusicId> {
    CURRENT.lock().as_ref().map(|(id, _)| *id)
}