use deps::tetra::audio::Sound;
use firecore_audio_lib::serialized::{SerializedMusicData, SerializedSoundData};
use deps::hash::HashMap;

use crate::audio::error::AddAudioError;

pub fn create() -> Result<(), AddAudioError> {
    *super::music::MUSIC_MAP.lock() = Some(HashMap::new());
    *super::sound::SOUND_MAP.lock() = Some(HashMap::new());
    Ok(())
}

pub fn add_track(music_data: SerializedMusicData) -> Result<(), AddAudioError> {
    let sound = Sound::from_file_data(&music_data.bytes);
    match super::music::MUSIC_MAP.lock().as_mut() {
        Some(map) => {
            map.insert(music_data.music.track, (music_data.music.data, sound));
            Ok(())
        }
        None => {
            Err(AddAudioError::Uninitialized)
        }
    }
}

pub fn add_sound(sound_data: SerializedSoundData) -> Result<(), AddAudioError> {
    let sound = Sound::from_file_data(&sound_data.bytes);
    match super::sound::SOUND_MAP.lock().as_mut() {
        Some(map) => {
            map.insert(sound_data.sound, sound);
            Ok(())
        }
        None => Ok(()), //Err(AddAudioError::Uninitialized),
    }
}