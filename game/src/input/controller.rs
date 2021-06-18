use crate::{
    deps::hash::HashSet,
    tetra::{
        input::{self, GamepadButton},
        Context,
    },
};

use enum_map::EnumMap;

use super::Control;

pub type ButtonSet = HashSet<GamepadButton>;
pub type ButtonMap = EnumMap<Control, GamepadButton>;

static mut BUTTON_CONTROLS: Option<ButtonMap> = None;

pub fn load(map: ButtonMap) {
    unsafe {
        BUTTON_CONTROLS = Some(map);
    }
}

pub fn pressed(ctx: &Context, control: Control) -> bool {
    unsafe { BUTTON_CONTROLS.as_ref() }
        .map(|controls| input::is_gamepad_button_pressed(ctx, 0, controls[control]))
        .unwrap_or_default()
}

pub fn down(ctx: &Context, control: Control) -> bool {
    unsafe { BUTTON_CONTROLS.as_ref() }
        .map(|controls| input::is_gamepad_button_down(ctx, 0, controls[control]))
        .unwrap_or_default()
}

pub fn default_button_map() -> ButtonMap {
    enum_map::enum_map! {
        Control::A => GamepadButton::A,
        Control::B => GamepadButton::B,
        Control::Up => GamepadButton::Up,
        Control::Down => GamepadButton::Down,
        Control::Left => GamepadButton::Left,
        Control::Right => GamepadButton::Right,
        Control::Start => GamepadButton::Start,
        Control::Select => GamepadButton::Back,
    }
}
