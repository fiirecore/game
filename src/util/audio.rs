use ahash::AHashMap as HashMap;
use kira::instance::handle::InstanceHandle;
use kira::manager::AudioManager;
use kira::sound::handle::SoundHandle;
use macroquad::prelude::info;
use crate::audio::music::Music;
use macroquad::prelude::warn;
use enum_iterator::IntoEnumIterator;
pub struct AudioContext {

    audio_manager: Option<AudioManager>,

    music_map: HashMap<Music, SoundHandle>,
    current_music: Option<InstanceHandle>,

    // sound_map: HashMap<u16, SoundId>,
    // current_sounds: Vec<InstanceId>,

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
            music_map: HashMap::new(),
            current_music: None,
        };

        this.bind_gamefreak();

        return this;
    }

    pub fn play_music(&mut self, music: Music) {
        if let Some(instance) = self.current_music.take() {
            stop_instance(music, instance);
        }
        match self.music_map.get_mut(&music) {
            Some(sound) => {
                match sound.play(kira::instance::InstanceSettings {
                    loop_start: kira::instance::InstanceLoopStart::Custom(music.loop_start().unwrap_or_default()),
                    ..Default::default()
                }) {
                    Ok(instance) => {
                        self.current_music = Some(instance);
                    }
                    Err(err) => warn!("Problem playing music {} with error {}", music, err),
                }
            }
            None => warn!("Could not get sound for {}", music),
        }        
    }

    pub fn is_music_playing(&self) -> bool {
        return self.current_music.is_some();
    }

    

    pub fn bind_music(&mut self) {
        match self.audio_manager.as_mut() {
            Some(manager) => {
                info!("Loading music...");
                for music in Music::into_enum_iter() {
                    if !self.music_map.contains_key(&music) {
                        match music.included_bytes() {
                            Some(bytes) => {
                                match crate::audio::loader::from_ogg_bytes(bytes, kira::sound::SoundSettings::default()) {
                                    Ok(sound) => {
                                        match manager.add_sound(sound) {
                                            Ok(sound) => {
                                                self.music_map.insert(music, sound);
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
                                            self.music_map.insert(music, sound);
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
                        self.music_map.insert(Music::IntroGamefreak, sound);
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

fn stop_instance(audio: impl std::fmt::Display, mut instance: kira::instance::handle::InstanceHandle) {
    if let Err(err) = instance.stop(kira::instance::StopInstanceSettings::default().fade_tween(kira::parameter::tween::Tween::linear(0.75))) {
        warn!("Problem stopping audio instance {} with error {}", audio, err);
    }
}