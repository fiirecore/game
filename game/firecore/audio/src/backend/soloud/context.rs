use deps::hash::HashMap;
use firecore_audio_lib::serialized::SerializedMusicData;
use parking_lot::Mutex;
use parking_lot::const_mutex;
use soloud::AudioExt;
use soloud::LoadExt;
use soloud::Soloud;
use soloud::WavStream;

use crate::error::AddAudioError;
use crate::error::Lockable;

use super::music::MUSIC;

pub(crate) static CONTEXT: Mutex<Option<Soloud>> = const_mutex(None);

pub fn create() -> Result<(), AddAudioError> {
    let sl = Soloud::default()?;
    match CONTEXT.try_lock() {
        Some(mut context) => *context = Some(sl),
        None => return Err(AddAudioError::LockError(Lockable::AudioManager)),
    }
    match MUSIC.try_lock() {
        Some(mut map) => *map = Some(HashMap::new()),
        None => return Err(AddAudioError::LockError(Lockable::AudioMap)),
    }
    Ok(())
}


pub fn add_track(music_data: SerializedMusicData) -> Result<(), AddAudioError> {
    let mut stream = WavStream::default();
    stream.load_mem(music_data.bytes)?;
    match super::music::MUSIC.try_lock() {
        Some(mut map) => match map.as_mut() {
            Some(map) => {
                map.insert(music_data.music.track, super::ReadableStream(stream));
                Ok(())
            }
            None => Err(AddAudioError::Uninitialized),
        }
        None => Err(AddAudioError::LockError(Lockable::AudioMap)),
    }
}