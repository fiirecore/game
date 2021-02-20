use parking_lot::RwLock;
use macroquad::prelude::KeyCode;
use ahash::{AHashMap as HashMap, AHashSet as HashSet};

use super::Control;


lazy_static::lazy_static! {
    pub static ref KEY_CONTROLS: RwLock<HashMap<Control, HashSet<KeyCode>>> = RwLock::new(default()); // benchmark if parking_lot rwlock and Hash's hashmap are faster than dashmap
}

pub fn pressed(control: &Control) -> bool {
    if let Some(keys) = KEY_CONTROLS.read().get(control) {
        for key in keys {
            if macroquad::prelude::is_key_pressed(*key) {
                return true;
            }
        }
    }
    return false;
}

pub fn down(control: &Control) -> bool {
    if let Some(keys) = KEY_CONTROLS.read().get(&control) {
        for key in keys {
            if macroquad::prelude::is_key_down(*key) {
                return true;
            }
        }
    }
    return false;
}

pub fn reload_with_config(config: &crate::io::data::configuration::Configuration) {
    *KEY_CONTROLS.write() = config.controls.clone();
}

pub fn default() -> HashMap<Control, HashSet<KeyCode>> {
    let mut controls = HashMap::new();
    controls.insert(Control::A, keyset(&[KeyCode::X]));
    controls.insert(Control::B, keyset(&[KeyCode::Z]));
    controls.insert(Control::Up, keyset(&[KeyCode::Up]));
    controls.insert(Control::Down, keyset(&[KeyCode::Down]));
    controls.insert(Control::Left, keyset(&[KeyCode::Left]));
    controls.insert(Control::Right, keyset(&[KeyCode::Right]));
    controls.insert(Control::Start, keyset(&[KeyCode::A]));
    controls.insert(Control::Select, keyset(&[KeyCode::S]));
    controls
}

pub fn keyset(codes: &[KeyCode]) -> HashSet<KeyCode> {
    let mut set = HashSet::new();
    for code in codes {
        set.insert(*code);
    }    
    return set;
}