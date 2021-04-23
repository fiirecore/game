use util::text::TextColor;
use macroquad::prelude::{Color, color_u8, Texture2D, draw_texture};
use util::hash::HashMap;
use crate::graphics::{byte_texture, draw};

pub static mut TEXT_RENDERER: Option<TextRenderer> = None;

pub struct TextRenderer {

    pub fonts: HashMap<u8, Font>,
    pub button: Texture2D,
    pub cursor: Texture2D,

}

impl TextRenderer {

    pub fn new() -> TextRenderer {
        TextRenderer {
            fonts: HashMap::new(),
            button: byte_texture(include_bytes!("../../assets/gui/button.png")),
            cursor: byte_texture(include_bytes!("../../assets/gui/cursor.png")),
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

    pub fn render_text_from_center(&self, font_id: u8, text: &str, color: Color, x: f32, y: f32) { // To - do: Have struct that stores a message, font id and color
        if let Some(font) = self.fonts.get(&font_id) {
            font.render_text_from_center(text, x, y, color);
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

    pub fn render_text_from_center(&self, text: &str, x: f32, y: f32, color: Color) {
        let mut len = 0.0;
        let x_offset = self.text_pixel_length(text) / 2.0;
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
const BLUE_COLOR: Color = color_u8!(48, 80, 200, 255);