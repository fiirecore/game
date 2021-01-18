use std::collections::HashMap;

use kira::manager::AudioManager;
use kira::manager::AudioManagerSettings;
use piston::Button;
use piston::Key;

use oorandom::Rand32 as Random;

use super::GameContext;

impl GameContext {

    pub fn new() -> GameContext {

        GameContext {

            keys: [0; 8],
            fkeys: [0; 12],

            keymap: HashMap::new(),
            fkeymap: HashMap::new(),

            random: Random::new(0),
            
            //app_console: AppConsole::new(),

            // audio: AudioContext,

            audio_manager: AudioManager::new(AudioManagerSettings::default()).unwrap(),
            
            music_map: HashMap::new(),
            current_music: None,

            sound_map: HashMap::new(),
            current_sounds: Vec::new(),

            battle_data: None,

        }
    }

    pub fn seed_random(&mut self, seed: u64) {
        self.random = Random::new(seed);
    }

    pub fn key_active(&self, index: usize) -> bool {
        self.keys[index] == 1 || self.keys[index] == 2
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

    

}