use firecore_util::text::TextColor;
use macroquad::prelude::{Color, color_u8};

pub mod map;
pub mod player;
pub mod font;
pub mod world;

pub trait IntoMQColor {

    fn into_color(self) -> Color;

}

impl IntoMQColor for TextColor {
    fn into_color(self) -> Color {
        match self {
            TextColor::White => WHITE_COLOR,
            TextColor::Gray => macroquad::prelude::GRAY,
            TextColor::Black => BLACK_COLOR,
            TextColor::Red => macroquad::prelude::RED,
            TextColor::Blue => BLUE_COLOR,
        }
    }
}

const WHITE_COLOR: Color = color_u8!(240, 240, 240, 255);
const BLACK_COLOR: Color = color_u8!(20, 20, 20, 255);
const BLUE_COLOR: Color = color_u8!(48, 80, 200, 255); // 48, 80, 200