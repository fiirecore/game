pub use ahash::AHashMap as HashMap;
use macroquad::prelude::{Color, Texture2D, draw_texture};

pub struct Font {

    pub font_width: u8,
    pub font_height: u8,

    pub chars: HashMap<char, Texture2D>,
    //custom_chars: HashMap<char, CustomChar>,

}

impl Font {

    pub fn render_text_from_left(&self, text: &str, x: f32, y: f32, color: Color) {
        let mut len: u32 = 0;
        for character in text.chars() {
            len += if let Some(texture) = self.chars.get(&character) {
                draw_texture(*texture, x + len as f32, y, color);
                texture.width() as u32
            } else {
                self.font_width as u32
            };       
        }
    }

    pub fn render_text_from_right(&self, text: &str, x: f32, y: f32, color: Color) {
        let mut len = 0.0;
        let x_offset = self.text_pixel_length(text);
        for character in text.chars() {
            len += if let Some(texture) = self.chars.get(&character) {
                draw_texture(*texture, x - x_offset + len, y, color);
                texture.width()
            } else {
                self.font_width as f32
            };
        }
    }

    pub fn text_pixel_length(&self, text: &str) -> f32 {
        text.chars().map(|character| {
            match self.chars.get(&character) {
                Some(texture) => texture.width(),
                None => self.font_width as f32,
            }
        }).sum()
    }

}