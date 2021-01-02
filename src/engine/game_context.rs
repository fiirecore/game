use std::collections::HashMap;

use oorandom::Rand32;
use piston::{Button, Key};

use crate::io::data::player_data::SavedPokemon;
use crate::io::{app_console::AppConsole, data::configuration::Configuration};

pub struct GameContext {

    pub configuration: Configuration,
    
    pub app_console: AppConsole,
    
    pub random: Rand32,

    pub keys: [usize; 8],
    pub fkeys: [usize; 12],

    pub keymap: HashMap<Button, usize>,
    pub fkeymap: HashMap<Button, usize>,

    pub battle: Option<SavedPokemon>,

}

impl GameContext {

    pub fn new(configuration: Configuration) -> GameContext {

        GameContext {

            keys: [0; 8],
            fkeys: [0; 12],

            keymap: HashMap::new(),
            fkeymap: HashMap::new(),

            configuration: configuration,

            app_console: AppConsole::new(),

            random: Rand32::new(0),

            battle: None,

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

}


    /*

    pub fn bind_sound<P>(&mut self, index: u8, path: P) where P: AsRef<Path> {
        let path = path.as_ref();

        if !self.sound_bank.contains_key(&index) {
            let sound = Source::new(&self.audio_context, path);
            match sound {
                Some(sound) => {
                    self.sound_bank.insert(index, sound);
                }
                None => {

                }
            }
            
        }
    }

    pub fn play_sound(&mut self, index: u8) {
        let sound = self.sound_bank.get_mut(&index);
        match sound {
            Some(sound) => {
                sound.play_detached();//.play();
            }
            None => {

            }
        }
    }

    pub fn play_music(&mut self, index: u8, loop_start: Duration) {
        
    }

}

struct Sound {

    decoder: Decoder<BufReader<File>>,

}

impl Sound {

    pub fn new<P>(path: P) -> Sound where P: AsRef<Path> {
        let path = path.as_ref();

        let file = File::open(path).unwrap();

        let d = rodio::Decoder::new(BufReader::new(file)).unwrap();

        Sound {

            decoder: d,

        }

    }

    pub fn play(&self, player: &mut RodioAudioContext) {
        player.device.1.play_raw(self.decoder.convert_samples());
    }

}

*/