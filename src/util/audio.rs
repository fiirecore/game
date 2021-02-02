use kira::manager::AudioManager;
use macroquad::prelude::info;
use crate::audio::music::Music;
use macroquad::prelude::warn;
use enum_iterator::IntoEnumIterator;


pub struct AudioContext {

    audio_manager: Option<AudioManager>,

}

impl AudioContext {

    pub fn new() -> Self {
        let mut this = Self {
            audio_manager: match AudioManager::new(kira::manager::AudioManagerSettings::default()) {
                Ok(am) => Some(am),
                Err(err) => {
                    warn!("Failed to create audio manager with error {}", err);
                    None
                },
            },
        };

        this.bind_gamefreak();

        return this;
    }    

    pub fn bind_music(&mut self) {
        match self.audio_manager.as_mut() {
            Some(manager) => {
                info!("Loading music...");
                for music in Music::into_enum_iter() {
                    let music_map = &mut crate::audio::context::music::MUSIC_CONTEXT.lock().music_map;
                    if !music_map.contains_key(&music) {
                        match music.included_bytes() {
                            Some(bytes) => {
                                match crate::audio::loader::from_ogg_bytes(bytes, kira::sound::SoundSettings::default()) {
                                    Ok(sound) => {
                                        match manager.add_sound(sound) {
                                            Ok(sound) => {
                                                music_map.insert(music, sound);
                                                info!("Loaded {} successfully", music);
                                            }
                                            Err(err) => warn!("Problem loading music {} with error {}", music, err),
                                        }
                                    }
                                    Err(err) => warn!("Problem decoding {} bytes in executable with error {}", music, err),
                                }
                                
                            }
                            None => {
                                if !(cfg!(debug_assertions) || cfg!(target_arch = "wasm32")) {
                                    match manager.load_sound(String::from("music/") + &music.to_string() + ".ogg", kira::sound::SoundSettings::default()) {
                                        Ok(sound) => {
                                            music_map.insert(music, sound);
                                            info!("Loaded {} successfully", music);
                                        }
                                        Err(err) => warn!("Problem loading music {} with error {}", music, err),
                                    }
                                }
                            }
                        }                        
                    }
                }
                info!("Finished loading world music!");
            }
            None => {}
        }
    }

    pub fn bind_gamefreak(&mut self) {
        match self.audio_manager.as_mut() {
            Some(manager) => {
                match manager.load_sound("music/gamefreak.ogg", kira::sound::SoundSettings::default()) {
                    Ok(sound) => {
                        crate::audio::context::music::MUSIC_CONTEXT.lock().music_map.insert(Music::IntroGamefreak, sound);
                    },
                    Err(err) => {
                        warn!("Could not load gamefreak intro music with error {}", err);
                    }
                }
            }
            None => {
                warn!("Could not bind gamefreak music due to missing audio manager!");
            }
        }
    }

}