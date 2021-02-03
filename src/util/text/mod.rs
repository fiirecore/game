use macroquad::prelude::Color;

use crate::io::data::text::font::FontSheetData;
use crate::util::graphics::Texture;
use crate::util::graphics::draw;

use self::font::Font;

use super::graphics::texture::byte_texture;

pub mod font;

pub struct TextRenderer {

    pub fonts: [Font; 3],
    pub button: Texture,
    pub cursor: Texture,

}

impl TextRenderer {

    pub fn new() -> TextRenderer {
        TextRenderer {
            fonts: [
                FontSheetData::open_sheet("fonts/font0.json").expect("Could not load font sheet 0"),
                FontSheetData::open_sheet("fonts/font1.json").expect("Could not load font sheet 1"),
                FontSheetData::open_sheet("fonts/font2.json").expect("Could not load font sheet 2") // Font 2 deprecated
            ],
            button: byte_texture(include_bytes!("../../../build/assets/gui/button.png")),
            cursor: byte_texture(include_bytes!("../../../build/assets/gui/cursor.png")),
        }
    }

    pub fn render_text_from_left(&self, font_id: usize, text: &str, color: Color, x: f32, y: f32) {
        self.fonts[font_id].render_text_from_left(text, x, y, color);
    }

    pub fn render_text_from_right(&self, font_id: usize, text: &str, color: Color, x: f32, y: f32) { // To - do: Have struct that stores a message, font id and color
        self.fonts[font_id].render_text_from_right(text, x, y, color);
    }

    pub fn render_button(&self, text: &str, font_id: usize, x: f32, y: f32) {
        draw(self.button, x + self.fonts[font_id].text_pixel_length(text) as f32, y);
    }

    pub fn render_cursor(&self, x: f32, y: f32) {
        draw(self.cursor, x, y);
    }

}