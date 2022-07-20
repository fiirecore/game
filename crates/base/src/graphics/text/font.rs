use fiirengine::{
    graphics::{DrawParams, Texture},
    Context,
};

use crate::{text::FontId, utils::HashMap};

pub type Fonts = HashMap<FontId, Font>;
pub type FontDimensions = u8;

pub type CharTextures = HashMap<char, Texture>;

pub struct Font {
    pub width: FontDimensions,
    pub height: FontDimensions,
    pub chars: CharTextures,
}

impl Font {
    pub fn draw_text_left(
        &self,
        ctx: &mut Context,
        text: &str,
        x: f32,
        y: f32,
        params: DrawParams,
    ) {
        let mut len = 0.0;
        for character in text.chars() {
            len += if let Some(texture) = self.chars.get(&character) {
                texture.draw(ctx, x + len, y, params);
                texture.width()
            } else {
                self.width as _
            };
        }
    }

    pub fn draw_text_right(
        &self,
        ctx: &mut Context,
        text: &str,
        x: f32,
        y: f32,
        params: DrawParams,
    ) {
        let mut len = 0.0;
        let x = x - self.text_pixel_length(text);
        for character in text.chars() {
            len += if let Some(texture) = self.chars.get(&character) {
                texture.draw(ctx, x + len, y, params);
                texture.width()
            } else {
                self.width as _
            };
        }
    }

    pub fn draw_text_center(
        &self,
        ctx: &mut Context,
        text: &str,
        center_vertical: bool,
        x: f32,
        y: f32,
        params: DrawParams,
    ) {
        let mut len = 0.0;

        let x_offset = (text
            .chars()
            .map(|ref character| match self.chars.get(character) {
                Some(texture) => texture.width() as FontDimensions,
                None => self.width,
            })
            .sum::<FontDimensions>()
            >> 1) as f32;

        let y_offset = if center_vertical {
            (self.height >> 1) as f32
        } else {
            0.0
        };

        for character in text.chars() {
            len += match self.chars.get(&character) {
                Some(texture) => {
                    texture.draw(ctx, x - x_offset + len, y - y_offset, params);
                    texture.width() as f32
                }
                None => self.width as f32,
            };
        }
    }

    pub fn text_pixel_length(&self, text: &str) -> f32 {
        text.chars()
            .map(|character| match self.chars.get(&character) {
                Some(texture) => texture.width() as f32,
                None => self.width as f32,
            })
            .sum()
    }
}
