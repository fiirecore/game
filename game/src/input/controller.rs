use deps::hash::HashSet;
use enum_map::EnumMap;
use crate::tetra::{Context, input::{self, GamepadButton as Key}};


use super::Control;

pub type ButtonSet = HashSet<Key>;
pub type ButtonMap = EnumMap<Control, HashSet<Key>>;

static mut BUTTON_CONTROLS: Option<ButtonMap> = None;

pub fn load(map: ButtonMap) {
    unsafe { BUTTON_CONTROLS = Some(map); }
}

pub fn pressed(ctx: &Context, control: Control) -> bool {
    if let Some(buttons) = buttons(control) {
        for button in buttons {
            if input::is_gamepad_button_pressed(ctx, 0, *button) {
                return true;
            }
        }
    }
    false
}

pub fn down(ctx: &Context, control: Control) -> bool {
    if let Some(buttons) = buttons(control) {
        for button in buttons {
            if input::is_gamepad_button_down(ctx, 0, *button) {
                return true;
            }
        }
    }
    false
}

#[inline]
pub fn buttons(control: Control) -> Option<&'static ButtonSet> {
    unsafe {
        BUTTON_CONTROLS.as_ref().map(|controls| &controls[control])//.unwrap_or_else(|| panic!("Could not get keys for control {:?}!", control)))
    }
}

pub fn default_button_map() -> ButtonMap {
    enum_map::enum_map! {
        Control::A => keyset(&[Key::A]),
        Control::B => keyset(&[Key::B]),
        Control::Up => keyset(&[Key::Up]),
        Control::Down => keyset(&[Key::Down]),
        Control::Left => keyset(&[Key::Left]),
        Control::Right => keyset(&[Key::Right]),
        Control::Start => keyset(&[Key::Start]),
        Control::Select => keyset(&[Key::Back]),
    }
}

fn keyset(
    codes: &[Key]
) -> ButtonSet {
    let mut set = HashSet::new();
    for code in codes {
        set.insert(*code);
    }    
    set
}