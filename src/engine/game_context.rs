use std::collections::HashMap;
use std::path::Path;

use kira::instance::InstanceId;
use kira::instance::InstanceSettings;
use kira::instance::StopInstanceSettings;
use kira::manager::AudioManager;
use kira::manager::AudioManagerSettings;
use kira::playable::PlayableSettings;
use kira::sound::SoundId;
use log::warn;
use oorandom::Rand32;
use piston::{Button, Key};

use crate::audio::music::Music;
use crate::battle::battle_context::BattleContext;
use crate::io::data::configuration::Configuration;
use crate::util::file_util::asset_as_pathbuf;

pub struct GameContext {

    pub configuration: Configuration,

    pub keys: [usize; 8],
    pub fkeys: [usize; 12],

    pub keymap: HashMap<Button, usize>,
    pub fkeymap: HashMap<Button, usize>,
    
    pub random: Rand32,

    //pub app_console: AppConsole,
    
    pub audio_manager: AudioManager,

    pub music_map: HashMap<Music, SoundId>,
    current_music: Option<InstanceId>,

    pub sound_map: HashMap<u16, SoundId>,
    current_sounds: Vec<InstanceId>,
    
    pub battle_context: BattleContext,

    pub save_data: bool,

}

impl GameContext {

    pub fn new(configuration: Configuration) -> GameContext {

        GameContext {

            configuration: configuration,

            keys: [0; 8],
            fkeys: [0; 12],

            keymap: HashMap::new(),
            fkeymap: HashMap::new(),

            random: Rand32::new(0),
            
            //app_console: AppConsole::new(),


            audio_manager: AudioManager::new(AudioManagerSettings::default()).unwrap(),
            
            music_map: HashMap::new(),
            current_music: None,

            sound_map: HashMap::new(),
            current_sounds: Vec::new(),

            battle_context: BattleContext::empty(),

            save_data: false,

        }
    }

    pub fn seed_random(&mut self, seed: u64) {
        self.random = Rand32::new(seed);
    }

    pub fn reload_config(&mut self) {
        self.configuration.reload();
    }

    pub(crate) fn fill_keymaps(&mut self, keys: Vec<Key>) {

        let mut count = 0;

        for key in keys {
            self.keymap.insert(Button::Keyboard(key), count);
            count+=1;
        }

        let fkeys = vec![Key::F1, Key::F2, Key::F3, Key::F4, Key::F5, Key::F6, Key::F7, Key::F8, Key::F9, Key::F10, Key::F11, Key::F12];

        count = 0;

        for key in fkeys {
            self.fkeymap.insert(Button::Keyboard(key), count);
            count+=1;
        }

    }

    pub fn load_music(&mut self, music: Music) {
        let mut path = music.to_string();
        path.push_str(".ogg");
        match self.audio_manager.load_sound(asset_as_pathbuf("audio/music").join(path), PlayableSettings {
            default_loop_start: Some(0.0),
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