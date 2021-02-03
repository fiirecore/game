pub use ahash::AHashMap as HashMap;
use macroquad::prelude::Color;
use macroquad::prelude::draw_texture;
pub use crate::util::graphics::Texture;

pub struct Font {

    pub font_width: u8,
    pub font_height: u8,

    pub chars: HashMap<char, Texture>,
    //custom_chars: HashMap<char, CustomChar>,

}

impl Font {

    pub fn render_text_from_left(&self, text: &str, x: f32, y: f32, color: Color) {
        let mut len: u32 = 0;
        for character in text.chars() {
            len += match self.chars.get(&character) {
                Some(texture) => {
                    draw_texture(*texture, x + len as f32, y, color);
                    texture.width() as u32
                },
                None => {
                    self.chars.values().next().unwrap().width() as u32
                }
            };          
        }
    }

    pub fn render_text_from_right(&self, text: &str, x: f32, y: f32, color: Color) {
        let mut len = 0.0;
        let x_offset = self.text_pixel_length(text) as f32;
        for character in text.chars() {
            len += match self.chars.get(&character) {
                Some(texture) => {
                    draw_texture(*texture, x - x_offset + len as f32, y, color);
                    texture.width()
                },
                None => {
                    self.chars.values().next().unwrap().width()
                }
            };         
        }
    }

    pub fn text_pixel_length(&self, text: &str) -> f32 {
        text.chars().map(|character| {
            match self.chars.get(&character) {
                Some(texture) => texture.width(),
                None => self.chars.values().next().unwrap().width(),
            }
        }).sum()
    }

}