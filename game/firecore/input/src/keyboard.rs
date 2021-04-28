use firecore_util::hash::{HashMap, HashSet};

use macroquad::prelude::KeyCode;

use super::Control;

pub type KeySet = HashSet<KeyCode>;
pub type KeyMap = HashMap<Control, HashSet<KeyCode>>;

static mut KEY_CONTROLS: Option<KeyMap> = None;

pub fn load(key_map: KeyMap) {
    unsafe { KEY_CONTROLS = Some(key_map); }
}

pub fn pressed(control: &Control) -> bool {
    if let Some(keys) = keys(control) {
        for key in keys {
            if macroquad::prelude::is_key_pressed(*key) {
                return true;
            }
        }
    }
    false
}

pub fn down(control: &Control) -> bool {
    if let Some(keys) = keys(control) {
        for key in keys {
            if macroquad::prelude::is_key_down(*key) {
                return true;
            }
        }
    }
    false
}

pub fn keys(control: &Control) -> Option<&KeySet> {
    unsafe {
        KEY_CONTROLS.as_ref().map(|controls| controls.get(control).unwrap_or_else(|| panic!("Could not get keys for control {:?}!", control)))
    }
}

pub fn default_key_map() -> HashMap<Control, KeySet> {

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

fn keyset(
    codes: &[KeyCode]
) -> KeySet {
    let mut set = HashSet::new();
    for code in codes {
        set.insert(*code);
    }    
    set
}