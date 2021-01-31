use ahash::AHashMap as HashMap;
use kira::instance::handle::InstanceHandle;
use kira::manager::AudioManager;
use kira::sound::handle::SoundHandle;
use macroquad::prelude::info;
use crate::audio::Music;
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
        let mut am = match AudioManager::new(kira::manager::AudioManagerSettings::default()) {
            Ok(am) => Some(am),
            Err(err) => {
                warn!("Failed to create audio manager with error {}", err);
                None
            },
        };

        Self::bind_gamefreak(&mut am);

        Self {
            audio_manager: am,
            music_map: HashMap::new(),
            current_music: None,
        }
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
        // if cfg!(not(target_arch = "wasm32")) {
        //         std::thread::spawn( || {
        //             Self::bind_music_main(&mut self.audio_manager, &mut self.music_map);        
        //         });
        // } else {
            Self::bind_music_main(&mut self.audio_manager, &mut self.music_map);
        //}
    }

    fn bind_music_main(manager: &mut Option<AudioManager>, music_map: &mut HashMap<Music, SoundHandle>) {
        match manager {
            Some(manager) => {
                info!("Loading music...");
                for music in Music::into_enum_iter() {
                    if music != Music::IntroGamefreak {
                        match music.included_bytes() {
                            Some(bytes) => {
                                match crate::audio::from_ogg_bytes(bytes, kira::sound::SoundSettings::default()) {
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
                                if !cfg!(debug_assertions) {
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

    pub fn bind_gamefreak(audio_manager: &mut Option<AudioManager>) {
        match audio_manager {
            Some(manager) => {
                match manager.load_sound("music/gamefreak.ogg", kira::sound::SoundSettings::default()) {
                    Ok(sound) => return *crate::audio::music::GAMEFREAK_MUSIC.lock() = Some(sound),
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