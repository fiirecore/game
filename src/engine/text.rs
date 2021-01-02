use std::collections::HashMap;
use opengl_graphics::{GlGraphics, ImageSize, Texture};
use piston_window::Context;

use crate::util::render_util::{draw, draw_o};

pub struct TextRenderer {

    pub(crate) fonts: Vec<HashMap<char, Texture>>,
    pub(crate) button: Option<Texture>,
    pub(crate) cursor: Option<Texture>,

}

pub static TEXT_WIDTH0: u8 = 5;
pub static TEXT_HEIGHT0: u8 = 12;

pub static TEXT_WIDTH1: u8 = 6;
pub static TEXT_HEIGHT1: u8 = 14;

impl TextRenderer {

    pub fn new() -> TextRenderer {
        TextRenderer {
            fonts: Vec::new(),
            button: None,
            cursor: None,
        }
    }

    pub fn render_text_from_left(&self, ctx: &mut Context, g: &mut GlGraphics, font_id: usize, text: &str, x: isize, y: isize) {
        let mut len: u32 = 0;
        for character in text.chars() {
            match self.fonts[font_id].get(&character) {
                Some(texture) => {
                    draw(ctx, g, texture, x + len as isize, y);
                    len += texture.get_width();
                },
                None => {
                    len += self.fonts[font_id].values().next().unwrap().get_width();
                }
            }            
        }
    }

    pub fn render_text_from_right(&self, ctx: &mut Context, g: &mut GlGraphics, font_id: usize, text: &str, x: isize, y: isize) {
        let mut len: u32 = 0;
        let x_offset = self.text_pixel_length(font_id, text);
        for character in text.chars() {
            match self.fonts[font_id].get(&character) {
                Some(texture) => {
                    draw(ctx, g, texture, x - x_offset as isize + len as isize, y);
                    len += texture.get_width();
                },
                None => {
                    len += self.fonts[font_id].values().next().unwrap().get_width();
                }
            }            
        }
    }

    pub fn text_pixel_length(&self, font_id: usize, text: &str) -> u32 {
        let mut len = 0;
        for character in text.chars() {
            if let Some(texture) = self.fonts[font_id].get(&character) {
                len += texture.get_width();
            } else {
                len += self.fonts[font_id].values().next().unwrap().get_width();
            }
        }
        return len;
    }

    pub fn render_button(&self, ctx: &mut Context, g: &mut GlGraphics, text: &str, font_id: usize, offset: i8, x: isize, y: isize) {
        let len = self.text_pixel_length(font_id, text) as isize;
        draw_o(ctx, g, &self.button, x + len, y + offset as isize + 5);
    }

    pub fn render_cursor(&self, ctx: &mut Context, g: &mut GlGraphics, x: isize, y: isize) {
        draw_o(ctx, g, &self.cursor, x, y);
    }

}