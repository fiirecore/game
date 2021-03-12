use dashmap::DashMap;
use macroquad::prelude::Color;

use crate::io::data::font::FontSheetData;
use crate::util::graphics::Texture;
use crate::util::graphics::draw;

use self::font::Font;

use super::graphics::texture::byte_texture;

pub mod font;

lazy_static::lazy_static! {
    pub static ref FONTS: DashMap<usize, Font> = DashMap::new();
}

pub async fn load() {
    FontSheetData::open_sheet("assets/fonts/font0.ron").await;
    FontSheetData::open_sheet("assets/fonts/font1.ron").await;
    FontSheetData::open_sheet("assets/fonts/font2.ron").await; // Font 2 deprecated
}

pub struct TextRenderer {

    // pub fonts: [Font; 3],
    pub button: Texture,
    pub cursor: Texture,

}

impl TextRenderer {

    pub fn new() -> TextRenderer {
        TextRenderer {
            button: byte_texture(include_bytes!("../../../build/assets/gui/button.png")),
            cursor: byte_texture(include_bytes!("../../../build/assets/gui/cursor.png")),
        }
    }

    pub fn render_text_from_left(&self, font_id: usize, text: &str, color: Color, x: f32, y: f32) {
        if let Some(font) = FONTS.get(&font_id) {
            font.render_text_from_left(text, x, y, color);
        }
    }

    pub fn render_text_from_right(&self, font_id: usize, text: &str, color: Color, x: f32, y: f32) { // To - do: Have struct that stores a message, font id and color
        if let Some(font) = FONTS.get(&font_id) {
            font.render_text_from_right(text, x, y, color);
        }
    }

    pub fn render_button(&self, text: &str, font_id: usize, x: f32, y: f32) {
        if let Some(font) = FONTS.get(&font_id) {
            draw(self.button, x + font.text_pixel_length(text) as f32, y + 2.0);
        }
    }

    pub fn render_cursor(&self, x: f32, y: f32) {
        draw(self.cursor, x, y);
    }

}