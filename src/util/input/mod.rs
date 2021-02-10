use ahash::AHashMap as HashMap;
use ahash::AHashSet as HashSet;
use parking_lot::RwLock;
use macroquad::prelude::KeyCode;

pub use self::control::Control;

mod control;

lazy_static::lazy_static! {
    static ref KEY_CONTROLS: RwLock<HashMap<Control, HashSet<KeyCode>>> = RwLock::new(Control::default_map()); // benchmark if parking_lot rwlock and Hash's hashmap are faster than dashmap
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

// #[derive(serde::Serialize, serde::Deserialize)]
// #[serde(remote = "KeyCode")]
// pub enum KeyCodeDef {

// }