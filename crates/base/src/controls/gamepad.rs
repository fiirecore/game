use enum_map::EnumMap;

use crate::{
    input::gamepad::{self, button::Button},
    utils::HashSet,
    Context, EngineContext,
};

use super::Control;

pub type ButtonSet = HashSet<Button>;
pub type ButtonMap = EnumMap<Control, Button>;

pub fn pressed(ctx: &Context, eng: &EngineContext, control: Control) -> bool {
    gamepad::gamepads(ctx)
        .next()
        .map(|gamepad| gamepad::button::pressed(ctx, gamepad, eng.controls.controller[control]))
        .unwrap_or_default()
}

pub fn down(ctx: &Context, eng: &EngineContext, control: Control) -> bool {
    gamepad::gamepads(ctx)
        .next()
        .map(|gamepad| gamepad::button::down(ctx, gamepad, eng.controls.controller[control]))
        .unwrap_or_default()
}

pub fn default_button_map() -> ButtonMap {
    enum_map::enum_map! {
        Control::A => Button::South,
        Control::B => Button::East,
        Control::Up => Button::DPadUp,
        Control::Down => Button::DPadDown,
        Control::Left => Button::DPadLeft,
        Control::Right => Button::DPadRight,
        Control::Start => Button::Start,
        Control::Select => Button::Select,
    }
}

pub fn set_button_map(ctx: &mut EngineContext, buttons: ButtonMap) {
    ctx.controls.controller = buttons;
}

pub fn get_bind(ctx: &EngineContext, control: Control) -> Button {
    ctx.controls.controller[control]
}

pub fn get_bind_mut(ctx: &mut EngineContext, control: Control) -> &mut Button {
    &mut ctx.controls.controller[control]
}
