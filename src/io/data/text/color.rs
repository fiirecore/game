use macroquad::color_u8;
use macroquad::prelude::Color;

#[derive(Debug, Copy, Clone, serde::Deserialize, serde::Serialize)]
pub enum TextColor {

    White,
    Gray,
    Black,
    Red,
    Blue,

}

impl Default for TextColor {
    fn default() -> Self {
        Self::Black
    }
}

impl Into<Color> for TextColor {
    fn into(self) -> Color {
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