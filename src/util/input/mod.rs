use serde::{Serialize, Deserialize};

pub mod keyboard;
//pub mod controller;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
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
    return false;
}

pub fn down(control: Control) -> bool {
    if keyboard::down(&control) {
        return true;
    }
    return false;
}