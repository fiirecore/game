use piston_window::{Button, Key};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ControlConfiguration {

    pub a: Vec<Button>,
    pub b: Vec<Button>,

    pub up: Vec<Button>,
    pub down: Vec<Button>,
    pub left: Vec<Button>,
    pub right: Vec<Button>,

    pub start: Vec<Button>,
    pub select: Vec<Button>,

}

impl Default for ControlConfiguration {
    fn default() -> Self {
        Self {
            a: vec![Button::Keyboard(Key::X)],
            b: vec![Button::Keyboard(Key::Z)],
            up: vec![Button::Keyboard(Key::Up)],
            down: vec![Button::Keyboard(Key::Down)],
            left: vec![Button::Keyboard(Key::Left)],
            right: vec![Button::Keyboard(Key::Right)],
            start: vec![Button::Keyboard(Key::A)],
            select: vec![Button::Keyboard(Key::S)],
        }
    }
}

impl ControlConfiguration {
    
}