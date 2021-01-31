use ahash::AHashMap as HashMap;
use ahash::AHashSet as HashSet;
use parking_lot::RwLock;
use macroquad::prelude::KeyCode;
use serde::{Deserialize, Serialize};

lazy_static::lazy_static! {
    static ref KEY_CONTROLS: RwLock<HashMap<Control, HashSet<KeyCode>>> = RwLock::new(HashMap::new()); // benchmark if parking_lot rwlock and Hash's hashmap are faster than dashmap
}

pub fn pressed(control: Control) -> bool {
    if let Some(keys) = KEY_CONTROLS.read().get(&control) {
        for key in keys {
            if macroquad::prelude::is_key_pressed(*key) {
                return true;
            }
        }
    }
    return false;
}

pub fn down(control: Control) -> bool {
    if let Some(keys) = KEY_CONTROLS.read().get(&control) {
        for key in keys {
            if macroquad::prelude::is_key_down(*key) {
                return true;
            }
        }
    }
    return false;
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum Control {

    A,
    B,
    Up,
    Down,
    Left,
    Right,
    Start,
    Select,
    //Escape,

}

pub(crate) fn default_keybinds() {
        
    let mut controls = KEY_CONTROLS.write();
    controls.insert(Control::A, set_of(&[KeyCode::X]));
    controls.insert(Control::B, set_of(&[KeyCode::Z]));
    controls.insert(Control::Up, set_of(&[KeyCode::Up]));
    controls.insert(Control::Down, set_of(&[KeyCode::Down]));
    controls.insert(Control::Left, set_of(&[KeyCode::Left]));
    controls.insert(Control::Right, set_of(&[KeyCode::Right]));
    controls.insert(Control::Start, set_of(&[KeyCode::A]));
    controls.insert(Control::Select, set_of(&[KeyCode::S]));
    //controls.insert(KeyCode::Escape), Control::Escape);
}

fn set_of(codes: &[KeyCode]) -> HashSet<KeyCode> {
    let mut set = HashSet::new();
    for code in codes {
        set.insert(*code);
    }    
    return set;
}

// #[derive(Clone, Copy, PartialEq)]
// pub enum ControlState {

//     Up,
//     Pressed,
//     Held

// }

// impl GameContext {

//     pub(crate) fn key_update(&mut self, control: Control) {
//         if self.input.controls[control] == ControlState::Pressed {
//             self.input.controls[control] = ControlState::Held;
//         }
//     }

//     pub(crate) fn key_press(&mut self, button: &Button) {
//         if let Some(control) = self.input.controls.get(button) {
//             self.input.controls[*control] = ControlState::Pressed;
//         }
//     }

//     pub(crate) fn key_release(&mut self, button: &Button) {
//         if let Some(control) = self.input.controls.get(button) {
//             self.input.controls[*control] = ControlState::Up;
//         }
//     }

// }