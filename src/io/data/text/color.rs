use macroquad::color_u8;
use macroquad::prelude::Color;

#[derive(Debug, Copy, Clone, serde::Deserialize)]
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