use std::collections::HashMap;

use oorandom::Rand32;
use piston::{Button, Key};

use crate::battle::battle_context::BattleContext;
use crate::io::data::configuration::Configuration;

pub struct GameContext {

    pub configuration: Configuration,
    
    //pub app_console: AppConsole,
    
    pub random: Rand32,

    pub keys: [usize; 8],
    pub fkeys: [usize; 12],

    pub keymap: HashMap<Button, usize>,
    pub fkeymap: HashMap<Button, usize>,

    pub battle_context: BattleContext,

    pub save_data: bool,

}

impl GameContext {

    pub fn new(configuration: Configuration) -> GameContext {

        GameContext {

            keys: [0; 8],
            fkeys: [0; 12],

            keymap: HashMap::new(),
            fkeymap: HashMap::new(),

            configuration: configuration,

            //app_console: AppConsole::new(),

            random: Rand32::new(0),

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

}