use enum_map::Enum;
use notan::prelude::{App, Plugins};
use serde::{Deserialize, Serialize};

// #[cfg(all(not(target_arch = "wasm32"), feature = "gamepad"))]
// pub mod gamepad;
pub mod context;
pub mod keyboard;
pub mod touchscreen;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize, Enum)]
pub enum Control {
    A,
    B,
    Up,
    Down,
    Left,
    Right,
    Start,
    Select,
}

pub fn pressed(app: &App, plugins: &Plugins, control: Control) -> bool {
    if keyboard::pressed(app, plugins, control) {
        return true;
    }
    if touchscreen::pressed(plugins, control) {
        return true;
    }
    // if gamepad::pressed(ctx, eng, control) {
    //     return true;
    // }
    false
}

pub fn down(app: &App, plugins: &Plugins, control: Control) -> bool {
    if keyboard::down(app, plugins, control) {
        return true;
    }
    if touchscreen::down(plugins, control) {
        return true;
    }
    // if gamepad::down(ctx, eng, control) {
    //     return true;
    // }
    false
}

impl Control {
    pub fn name(&self) -> &str {
        match self {
            Control::A => "A",
            Control::B => "B",
            Control::Up => "Up",
            Control::Down => "Down",
            Control::Left => "Left",
            Control::Right => "Right",
            Control::Start => "Start",
            Control::Select => "Select",
        }
    }
}
