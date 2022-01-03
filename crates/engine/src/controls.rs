use fiirengine::Context;
use enum_map::Enum;
use serde::{Deserialize, Serialize};

use crate::EngineContext;

#[cfg(all(not(target_arch = "wasm32"), feature = "gamepad"))]
pub mod gamepad;
pub mod keyboard;
// pub mod touchscreen;

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

pub(crate) struct ControlsContext {
    pub keyboard: keyboard::KeyMap,
    #[cfg(all(not(target_arch = "wasm32"), feature = "gamepad"))]
    pub controller: gamepad::ButtonMap,
    // pub touchscreen: touchscreen::Touchscreen,
}

impl Default for ControlsContext {
    fn default() -> Self {
        Self {
            keyboard: keyboard::default_key_map(),
            #[cfg(all(not(target_arch = "wasm32"), feature = "gamepad"))]
            controller: gamepad::default_button_map(),
        }
    }
}

pub fn pressed(ctx: &Context, eng: &EngineContext, control: Control) -> bool {
    if keyboard::pressed(ctx, eng, control) {
        return true;
    }
    #[cfg(all(not(target_arch = "wasm32"), feature = "gamepad"))]
    if gamepad::pressed(ctx, eng, control) {
        return true;
    }
    // if let Some(controls) = unsafe{touchscreen::TOUCHSCREEN.as_ref()} {
    //     if controls.pressed(&control) {
    //         return true;
    //     }
    // }
    false
}

pub fn down(ctx: &Context, eng: &EngineContext, control: Control) -> bool {
    if keyboard::down(ctx, eng, control) {
        return true;
    }
    #[cfg(all(not(target_arch = "wasm32"), feature = "gamepad"))]
    if gamepad::down(ctx, eng, control) {
        return true;
    }
    // if let Some(controls) = unsafe{touchscreen::TOUCHSCREEN.as_ref()} {
    //     if controls.down(&control) {
    //         return true;
    //     }
    // }
    false
}
