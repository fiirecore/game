use macroquad::prelude::Color;
use ahash::AHashMap as HashMap;
use macroquad::prelude::Texture2D;

use super::font::Font;

use crate::util::graphics::{byte_texture, draw};

// #[deprecated(note = "move to own crate")]
pub struct TextRenderer {

    pub fonts: HashMap<u8, Font>,
    pub button: Texture2D,
    pub cursor: Texture2D,

}

impl TextRenderer {

    pub fn new() -> TextRenderer {
        TextRenderer {
            fonts: HashMap::new(),
            button: byte_texture(include_bytes!("../../../build/assets/gui/button.png")),
            cursor: byte_texture(include_bytes!("../../../build/assets/gui/cursor.png")),
        }
    }

    pub fn render_text_from_left(&self, font_id: u8, text: &str, color: Color, x: f32, y: f32) {
        if let Some(font) = self.fonts.get(&font_id) {
            font.render_text_from_left(text, x, y, color);
        }
    }

    pub fn render_text_from_right(&self, font_id: u8, text: &str, color: Color, x: f32, y: f32) { // To - do: Have struct that stores a message, font id and color
        if let Some(font) = self.fonts.get(&font_id) {
            font.render_text_from_right(text, x, y, color);
        }
    }

    pub fn render_button(&self, text: &str, font_id: u8, x: f32, y: f32) {
        if let Some(font) = self.fonts.get(&font_id) {
            draw(self.button, x + font.text_pixel_length(text) as f32, y + 2.0);
        }
    }

    pub fn render_cursor(&self, x: f32, y: f32) {
        draw(self.cursor, x, y);
    }

}