use macroquad::prelude::{Color, color_u8, Rect, WHITE, Texture2D, DrawTextureParams, draw_rectangle, draw_texture_ex};

use crate::util::graphics::{byte_texture, draw};

pub struct Panel {

    pub rect: Rect,

    corner: Texture2D,
    // pub color: PanelColor,

}

impl Panel {

    const BORDER: Color = color_u8!(40, 48, 48, 255);
    const BACKGROUND: Color = color_u8!(248, 248, 248, 255);
    const LIGHT: Color = color_u8!(136, 216, 128, 255);
    const DARK: Color = color_u8!(72, 184, 64, 255);

    const BOTTOM_LEFT: DrawTextureParams = DrawTextureParams {
        rotation: 90.0.to_radians(),
        ..Default::default()
    };

    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            corner: byte_texture(include_bytes!("../../build/assets/gui/panel/corner.png")),
        }
    }

    pub fn render(&self) {
        draw_rectangle(self.rect.x + 6.0, self.rect.y + 6.0, self.rect.w - 12.0, self.rect.h - 12.0, Self::BACKGROUND);
        draw(self.corner, self.rect.x, self.rect.y);
        draw_texture_ex(self.corner, self.rect.x + self.rect.w - 6.0, self.rect.y, WHITE, Self::BOTTOM_LEFT);
    }

}

// pub enum PanelColor {

//     Gray,

// }

// impl Into<macroquad::prelude::Color> for PanelColor {
//     fn into(self) -> macroquad::prelude::Color {
//         match self {
//             Gray
//         }
//     }
// }