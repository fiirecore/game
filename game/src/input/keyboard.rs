use deps::hash::HashSet;
use enum_map::EnumMap;
use crate::tetra::{Context, input::{self, Key}};


use super::Control;

pub type KeySet = HashSet<Key>;
pub type KeyMap = EnumMap<Control, HashSet<Key>>;

static mut KEY_CONTROLS: Option<KeyMap> = None;

pub fn load(key_map: KeyMap) {
    unsafe { KEY_CONTROLS = Some(key_map); }
}

pub fn pressed(ctx: &Context, control: Control) -> bool {
    if let Some(keys) = keys(control) {
        for key in keys {
            if input::is_key_pressed(ctx, *key) {
                return true;
            }
        }
    }
    false
}

pub fn down(ctx: &Context, control: Control) -> bool {
    if let Some(keys) = keys(control) {
        for key in keys {
            if input::is_key_down(ctx, *key) {
                return true;
            }
        }
    }
    false
}

#[inline]
pub fn keys(control: Control) -> Option<&'static KeySet> {
    unsafe {
        KEY_CONTROLS.as_ref().map(|controls| &controls[control])//.unwrap_or_else(|| panic!("Could not get keys for control {:?}!", control)))
    }
}

pub fn default_key_map() -> KeyMap {
    enum_map::enum_map! {
        Control::A => keyset(&[Key::X]),
        Control::B => keyset(&[Key::Z]),
        Control::Up => keyset(&[Key::Up]),
        Control::Down => keyset(&[Key::Down]),
        Control::Left => keyset(&[Key::Left]),
        Control::Right => keyset(&[Key::Right]),
        Control::Start => keyset(&[Key::A]),
        Control::Select => keyset(&[Key::S]),
    }
}

fn keyset(
    codes: &[Key]
) -> KeySet {
    let mut set = HashSet::new();
    for code in codes {
        set.insert(*code);
    }    
    set
}