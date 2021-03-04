use kira::manager::AudioManager;
use macroquad::prelude::info;
use macroquad::prelude::warn;
use enum_iterator::IntoEnumIterator;
use parking_lot::Mutex;
use crate::music::Music;

pub mod music;
pub mod sound;

lazy_static::lazy_static! {
    pub static ref AUDIO_CONTEXT: AudioContext = AudioContext::default();
}

#[derive(Default)]
pub struct AudioContext {

    audio_manager: Mutex<Option<AudioManager>>,

}

impl AudioContext {

    pub fn load(&self) {
        *self.audio_manager.lock() = match AudioManager::new(kira::manager::AudioManagerSettings::default()) {
            Ok(am) => Some(am),
            Err(err) => {
                warn!("Failed to create audio manager with error {}", err);
                None
            },
        };

        self.bind_gamefreak();
    }    

    pub fn bind_music(&self) {
        info!("Loading music...");
                for music in Music::into_enum_iter() {
                    if !self::music::MUSIC_CONTEXT.music_map.contains_key(&music) {
                        if let Some(bytes) = music.included_bytes() {
                            match super::from_ogg_bytes(bytes, kira::sound::SoundSettings::default()) {
                                Ok(sound) => match self.audio_manager.lock().as_mut() {
                                    Some(manager) => {
                                        match manager.add_sound(sound) {
                                            Ok(sound) => {
                                                self::music::MUSIC_CONTEXT.music_map.insert(music, sound);
                                                info!("Loaded music \"{:?}\" successfully", music);
                                            }
                                            Err(err) => {
                                                warn!("Problem loading music \"{:?}\" with error {}", music, err);
                                            }
                                        }
                                    }
                                    None => {}
                                }
                                Err(err) => {
                                    warn!("Problem decoding bytes of \"{:?}\" in executable with error {}", music, err);
                                }
                            }
                        }
                    }
                }
                for music in Music::into_enum_iter() {
                    if music.included_bytes().is_none() {
                        if !self::music::MUSIC_CONTEXT.music_map.contains_key(&music) {
                            if !(cfg!(debug_assertions)) {
                                match self.audio_manager.lock().as_mut() {
                                    Some(manager) => match manager.load_sound(String::from("music/") + &music.file_name() + ".ogg", kira::sound::SoundSettings::default()) {
                                        Ok(sound) => {
                                            self::music::MUSIC_CONTEXT.music_map.insert(music, sound);
                                            info!("Loaded \"{:?}\" successfully", music);
                                        }
                                        Err(err) => {
                                            warn!("Problem loading music \"{:?}\" with error {}", music, err);
                                        }
                                    }
                                    None => {
                                        warn!("Could not get audio manager from audio context while loading music \"{:?}\"!", music);
                                    }
                                }
                            }
                        }
                    }
                }
                info!("Finished loading world music!");
        
    }

    pub fn bind_gamefreak(&self) {
        match self.audio_manager.lock().as_mut() {
            Some(manager) => {
                match super::from_ogg_bytes(Music::IntroGamefreak.included_bytes().unwrap(), kira::sound::SoundSettings::default()) {
                    Ok(sound) => match manager.add_sound(sound) {
                        Ok(sound) => {
                            self::music::MUSIC_CONTEXT.music_map.insert(Music::IntroGamefreak, sound);
                        },
                        Err(err) => {
                            warn!("Could not load gamefreak intro music with error {}", err);
                        }
                    }
                    Err(err) => {
                        warn!("Could not decode gamefreak into audio with error {}", err);
                    }
                }
            }
            None => {
                warn!("Could not bind gamefreak music due to missing audio manager!");
            }
        }
    }

}

fn stop_instance(name: impl std::fmt::Debug, mut instance: kira::instance::handle::InstanceHandle) {
    if let Err(err) = instance.stop(kira::instance::StopInstanceSettings::default().fade_tween(kira::parameter::tween::Tween::linear(0.75))) {
        macroquad::prelude::warn!("Problem stopping audio instance {:?} with error {}", name, err);
    }
}