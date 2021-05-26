extern crate firecore_dependencies as deps;

use deps::tetra::Context;
use enum_map::Enum;
use serde::{Serialize, Deserialize};

pub mod keyboard;
pub mod controller;
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

pub fn pressed(ctx: &Context, control: Control) -> bool {
    if keyboard::pressed(ctx, control) {
        return true;
    }
    if controller::pressed(ctx, control) {
        return true;
    }
    // if let Some(controls) = unsafe{touchscreen::TOUCHSCREEN.as_ref()} {
    //     if controls.pressed(&control) {
    //         return true;
    //     }
    // }
    false
}

pub fn down(ctx: &Context, control: Control) -> bool {
    if keyboard::down(ctx, control) {
        return true;
    }
    if controller::down(ctx, control) {
        return true;
    }
    // if let Some(controls) = unsafe{touchscreen::TOUCHSCREEN.as_ref()} {
    //     if controls.down(&control) {
    //         return true;
    //     }
    // }
    false
}

pub type DebugBind = deps::tetra::input::Key;

pub fn debug_pressed(ctx: &Context, bind: DebugBind) -> bool {
    deps::tetra::input::is_key_pressed(ctx, bind)
}

pub fn debug_down(ctx: &Context, bind: DebugBind) -> bool {
    deps::tetra::input::is_key_down(ctx, bind)
}