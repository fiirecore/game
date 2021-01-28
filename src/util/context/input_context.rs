use macroquad::prelude::KeyCode;

use super::GameContext;

impl GameContext {

    pub(crate) fn fill_keymaps(&mut self) {

        let fkeys = [KeyCode::F1, KeyCode::F2, KeyCode::F3, KeyCode::F4, KeyCode::F5, KeyCode::F6, KeyCode::F7, KeyCode::F8, KeyCode::F9, KeyCode::F10, KeyCode::F11, KeyCode::F12];

        for i in 0..fkeys.len() {
            self.fkeymap.insert(fkeys[i], i);
        }

    }

}