use serde::{Serialize, Deserialize};

pub mod keyboard;
// pub mod touchscreen;
//pub mod controller;

pub type KeySet = ahash::AHashSet<macroquad::prelude::KeyCode>;
pub type KeySetSerializable = ahash::AHashSet<keyboard::serialization::KeySerializable>;

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
    //Escape,

}

pub fn pressed(control: Control) -> bool {
    if keyboard::pressed(&control) {
        return true;
    }
    // if touchscreen::TOUCH_CONTROLS.pressed(&control) {
    //     return true;
    // }
    return false;
}

pub fn down(control: Control) -> bool {
    if keyboard::down(&control) {
        return true;
    }
    // if touchscreen::TOUCH_CONTROLS.down(&control) {
    //     return true;
    // }
    return false;
}