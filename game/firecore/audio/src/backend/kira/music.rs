use kira::instance::InstanceSettings;
use kira::instance::StopInstanceSettings;
use kira::instance::handle::InstanceHandle;
use kira::sound::handle::SoundHandle;
use util::hash::HashMap;
use parking_lot::Mutex;
use firecore_audio_lib::music::{MusicId, MusicData};
use parking_lot::const_mutex;

use crate::error::PlayAudioError;

pub static MUSIC_MAP: Mutex<Option<HashMap<MusicId, (MusicData, SoundHandle)>>> = const_mutex(None);
pub static CURRENT_MUSIC: Mutex<Option<(MusicId, InstanceHandle)>> = const_mutex(None);

pub fn play_music(music: MusicId) -> Result<(), PlayAudioError> {
    match CURRENT_MUSIC.try_lock() {
        Some(mut current) => {
            if let Some((_, mut instance)) = current.take() {
                if let Err(err) = instance.stop(StopInstanceSettings::default()) {
                    return Err(PlayAudioError::CurrentError(err));
                }
            }
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
                            match audio.play(InstanceSettings {
                                loop_start: kira::instance::InstanceLoopStart::Custom(loop_start),
                                ..Default::default()
                            }) {
                                Ok(instance) => {
                                    *current = Some((music, instance));
                                    Ok(())
                                }
                                Err(err) => {
                                    Err(PlayAudioError::PlayError(err))
                                }
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
            Err(PlayAudioError::Uninitialized)
        }
    }
       
}

pub fn get_current_music() -> Option<MusicId> {
    CURRENT_MUSIC.lock().as_ref().map(|(id, _)| *id)
}