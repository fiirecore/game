use macroquad::color_u8;
use macroquad::prelude::Color;
use serde::Deserialize;

pub mod font;

#[derive(Debug, Clone, Deserialize)]
pub struct Message {

    pub font_id: usize,
    pub message: Vec<String>,
    pub color: TextColor,
    pub no_pause: bool,

}

impl Default for Message {
    fn default() -> Self {
        Self {
            font_id: 1,
            color: TextColor::default(),
            message: Vec::new(),
            no_pause: true,
        }
    }
}

impl Message {

    pub fn new(message: Vec<String>, no_pause: bool,) -> Self {
        Self::with_color(message, no_pause, TextColor::default())
    }

    pub fn with_color(message: Vec<String>, no_pause: bool, color: TextColor) -> Self {
        Self {
            message,
            no_pause,
            color,
            ..Default::default()
        }
    }

}

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum TextColor {

    White,
    Gray,
    Black,
    Red,
    Blue,

}

impl Default for TextColor {
    fn default() -> Self {
        Self::White
    }
}

impl Into<Color> for TextColor {
    fn into(self) -> Color {
        match self {
            TextColor::White => macroquad::prelude::WHITE,
            TextColor::Gray => macroquad::prelude::GRAY,
            TextColor::Black => macroquad::prelude::BLACK,
            TextColor::Red => macroquad::prelude::RED,
            TextColor::Blue => BLUE_COLOR,
        }
    }
}

//const WHITE_COLOR: Color = Color::new(1.2, 1.2, 1.2, 1.0);
const BLUE_COLOR: Color = color_u8!(48, 80, 200, 255); // 48, 80, 200