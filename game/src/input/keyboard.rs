use crate::{
    deps::hash::HashSet,
    tetra::{
        input::{self, Key},
        Context,
    },
};

use enum_map::EnumMap;

use super::Control;

pub type KeySet = HashSet<Key>;
pub type KeyMap = EnumMap<Control, HashSet<Key>>;

static mut KEY_CONTROLS: Option<KeyMap> = None;

pub fn load(key_map: KeyMap) {
    unsafe {
        KEY_CONTROLS = Some(key_map);
    }
}

pub fn pressed(ctx: &Context, control: Control) -> bool {
    unsafe { KEY_CONTROLS.as_ref() }
        .map(|controls| {
            controls[control]
                .iter()
                .any(|key| input::is_key_pressed(ctx, *key))
        })
        .unwrap_or_default()
}

pub fn down(ctx: &Context, control: Control) -> bool {
    unsafe { KEY_CONTROLS.as_ref() }
        .map(|controls| {
            controls[control]
                .iter()
                .any(|key| input::is_key_down(ctx, *key))
        })
        .unwrap_or_default()
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

fn keyset(codes: &[Key]) -> KeySet {
    let mut set = HashSet::new();
    for code in codes {
        set.insert(*code);
    }
    set
}
