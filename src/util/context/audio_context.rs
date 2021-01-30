use std::path::Path;

use crate::audio::Music;
use crate::util::file::asset_as_pathbuf;

use super::GameContext;

use kira::instance::InstanceSettings;
use kira::instance::StopInstanceSettings;
use kira::playable::PlayableSettings;
use macroquad::prelude::warn;

// pub struct AudioContext {

//     audio_manager: AudioManager,

//     music_map: HashMap<Music, SoundId>,
//     loaded_music: Mutex<HashMap<Music, Sound>>,
//     current_music: Option<InstanceId>,

//     sound_map: HashMap<u16, SoundId>,
//     current_sounds: Vec<InstanceId>,

// }

// impl AudioContext {

//     pub fn bind(&mut self, music: Vec<Music>) {
//         match self.loaded_music.lock() {
//             Ok(map) => {
//                 thread::spawn(move || {
//                     for music in music {
//                         let mut path = music.to_string();
//                         path.push_str(".ogg");
//                         match Sound::from_ogg_file(path, PlayableSettings {
//                             semantic_duration: music.len(),
//                             default_loop_start: music.loop_start(),
//                             ..Default::default()
//                         }) {
//                             Ok(sound) => {
//                                 map.insert(music, sound);
//                             },
//                             Err(err) => {

//                             }
//                         }
                        
//                     }
//                 }).join().unwrap();  
//             }
//             Err(err) => {

//             }
//         }   
//         info!("Map length: {}", self.loaded_music.lock().unwrap().len());
//     }

// }

impl GameContext {

    pub fn load_music(&mut self, music: Music) {
        let mut path = music.to_string();
            path.push_str(".ogg");
        match self.audio_manager.load_sound(asset_as_pathbuf("audio/music").join(path), PlayableSettings {
            default_loop_start: music.loop_start(),
            semantic_duration: music.len(),
            ..Default::default()
        }) {
            Ok(sound) => {
                self.music_map.insert(music, sound);
            }
            Err(err) => {
                warn!("Problem loading sound {} with error {}", music.to_string(), err);
            }
        } 
    }

    pub fn play_music(&mut self, music: Music) {
        if let Some(music) = self.music_map.get(&music) {
            if self.current_music.is_some() {
                self.audio_manager.stop_instance(self.current_music.take().unwrap(), StopInstanceSettings::default()).unwrap();
            }
            self.current_music = Some(self.audio_manager.play(*music, InstanceSettings::default()).unwrap());
        } else {
            warn!("Could not get music for {}", music);
        }
    }

    pub fn stop_music(&mut self) {
        if self.current_music.is_some() {
            self.audio_manager.stop_instance(self.current_music.take().unwrap(), StopInstanceSettings::default()).unwrap();
        }
    }

    pub fn is_music_playing(&self) -> bool {
        self.current_music.is_some()
    }

    // pub async fn is_music_loaded(&self, music: Music) {}

    pub fn load_sound<P: AsRef<Path>>(&mut self, index: u16, path: P, settings: PlayableSettings) {
        self.sound_map.insert(index, self.audio_manager.load_sound(path, settings).unwrap());
    }

    pub fn play_sound(&mut self, index: &u16) { // add option to pause music
        if let Some(sound) = self.sound_map.get(index) {
            match self.audio_manager.play(*sound, InstanceSettings::default()) {
                Ok(instance) => {
                    self.current_sounds.push(instance);
                }
                Err(err) => {
                    warn!("{}", err);
                }
            }
        } else {
            warn!("Could not get sound at index #{}!", index);
        }
    }

}