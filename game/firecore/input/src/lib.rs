use serde::{Serialize, Deserialize};

pub mod keyboard;
pub mod touchscreen;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
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

pub fn pressed(control: Control) -> bool {
    if keyboard::pressed(&control) {
        return true;
    }
    if let Some(controls) = unsafe{touchscreen::TOUCHSCREEN.as_ref()} {
        if controls.pressed(&control) {
            return true;
        }
    }
    false
}

pub fn down(control: Control) -> bool {
    if keyboard::down(&control) {
        return true;
    }
    if let Some(controls) = unsafe{touchscreen::TOUCHSCREEN.as_ref()} {
        if controls.down(&control) {
            return true;
        }
    }
    false
}