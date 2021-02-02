use macroquad::prelude::Color;
use serde::Deserialize;

pub mod font;

#[derive(Debug, Deserialize)]
pub struct Message {

    pub font_id: usize,
    pub message: String,
    pub color: TextColor,

}

#[derive(Debug, Deserialize)]
pub enum TextColor {

    White,
    Black,
    Red,
    Blue,

}

impl Into<Color> for TextColor {
    fn into(self) -> Color {
        match self {
            TextColor::White => macroquad::prelude::WHITE,
            TextColor::Black => macroquad::prelude::GRAY,
            TextColor::Red => macroquad::prelude::RED,
            TextColor::Blue => macroquad::prelude::BLUE,
        }
    }
}