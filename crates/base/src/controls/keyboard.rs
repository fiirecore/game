use std::cell::RefMut;

use enum_map::EnumMap;

use notan::prelude::{App, KeyCode as Key, Plugins};

use super::{Control, context::ControlsContext};

// pub type KeySet = HashSet<Key>;
pub type KeyMap = EnumMap<Control, Key>;

pub fn pressed(app: &App, plugins: &Plugins, control: Control) -> bool {
    plugins
        .get::<ControlsContext>()
        .map(|ctx| app.keyboard.was_pressed(ctx.keyboard[control]))
        .unwrap_or_default()
}

pub fn down(app: &App, plugins: &Plugins, control: Control) -> bool {
    plugins
        .get::<ControlsContext>()
        .map(|ctx| app.keyboard.is_down(ctx.keyboard[control]))
        .unwrap_or_default()
}

pub fn default_key_map() -> KeyMap {
    enum_map::enum_map! {
        Control::A => Key::X,
        Control::B => Key::Z,
        Control::Up => Key::Up,
        Control::Down => Key::Down,
        Control::Left => Key::Left,
        Control::Right => Key::Right,
        Control::Start => Key::A,
        Control::Select => Key::S,
    }
}

pub fn set_key_map(plugins: &mut Plugins, keys: KeyMap) {
    match plugins.get_mut::<ControlsContext>() {
        Some(mut ctx) => ctx.keyboard = keys,
        None => todo!(),
    }
}

pub fn get_bind(plugins: &mut Plugins, control: Control) -> Key {
    match plugins.get_mut::<ControlsContext>() {
        Some(ctx) => ctx.keyboard[control],
        None => todo!(),
    }
}

pub fn get_bind_mut(plugins: &mut Plugins, control: Control) -> RefMut<Key> {
    match plugins.get_mut::<ControlsContext>() {
        Some(ctx) => RefMut::map(ctx, |ctx| &mut ctx.keyboard[control]),
        None => todo!(),
    }
}

// fn keyset(codes: &[Key]) -> KeySet {
//     let mut set = HashSet::with_capacity(codes.len());
//     for code in codes {
//         set.insert(*code);
//     }
//     set
// }
