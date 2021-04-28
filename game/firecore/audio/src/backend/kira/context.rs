use firecore_audio_lib::serialized::SerializedMusicData;
use firecore_audio_lib::serialized::SerializedSoundData;
use kira::manager::AudioManager;
use kira::sound::SoundSettings;
use parking_lot::Mutex;
use parking_lot::const_mutex;
use util::hash::HashMap;

use crate::error::AddAudioError;

pub static AUDIO_CONTEXT: Mutex<Option<AudioManager>> = const_mutex(None);

pub fn create() -> Result<(), kira::manager::error::SetupError> {
    *AUDIO_CONTEXT.lock() = match AudioManager::new(kira::manager::AudioManagerSettings::default()) {
        Ok(am) => Some(am),
        Err(err) => return Err(err),
    };

    *super::music::MUSIC_MAP.lock() = Some(HashMap::new());
    *super::sound::SOUND_MAP.lock() = Some(HashMap::new());

    Ok(())
}

pub fn add_track(music_data: SerializedMusicData) -> Result<(), AddAudioError> {
    match super::from_ogg_bytes(&music_data.bytes, SoundSettings::default()) {
        Ok(sound) => match AUDIO_CONTEXT.lock().as_mut() {
            Some(manager) => {
                match manager.add_sound(sound) {
                    Ok(sound) => {
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
                    Err(err) => Err(AddAudioError::ManagerAddError(err)),
                }
            }
            None => Err(AddAudioError::NoManager),
        }
        Err(err) => Err(AddAudioError::DecodeError(err)),
    }
}

pub fn add_sound(sound_data: SerializedSoundData) -> Result<(), AddAudioError> {
    match super::from_ogg_bytes(&sound_data.bytes, SoundSettings::default()) {
        Ok(sound) => {
            match super::context::AUDIO_CONTEXT.lock().as_mut() {
                Some(context) => {
                    match context.add_sound(sound) {
                        Ok(sound) => {
                            match super::sound::SOUND_MAP.lock().as_mut() {
                                Some(map) => {
                                    map.insert(sound_data.sound, sound);
                                    Ok(())
                                }
                                None => Err(AddAudioError::Uninitialized)
                            }
                        }
                        Err(err) => Err(AddAudioError::ManagerAddError(err)),
                    }
                }
                None => Err(AddAudioError::NoManager),
            }
        }
        Err(err) => Err(AddAudioError::DecodeError(err)),
    }
}