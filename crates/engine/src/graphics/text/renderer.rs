use fiirengine::{
    error::ImageError,
    graphics::{DrawParams, Image, Texture},
    Context,
};

use crate::{
    text::font::{CustomChar, FontId, FontSheet},
    utils::HashMap,
};

use super::font::{CharTextures, Font, FontDimensions, Fonts};

pub(crate) struct TextRenderer {
    fonts: Fonts,
    button: Texture,
    cursor: Texture,
}
impl TextRenderer {
    pub fn new(ctx: &mut Context) -> Result<Self, ImageError> {
        Ok(Self {
            fonts: Default::default(),
            button: Texture::new(ctx, include_bytes!("../../../assets/button.png"))?,
            cursor: Texture::new(ctx, include_bytes!("../../../assets/cursor.png"))?,
        })
    }

    pub fn add_font_sheet(
        &mut self,
        ctx: &mut Context,
        font_sheet: &FontSheet<Vec<u8>>,
    ) -> Result<(), ImageError> {
        self.fonts.insert(
            font_sheet.data.id,
            Font {
                width: font_sheet.data.width,
                height: font_sheet.data.height,
                chars: iterate_fontsheet(
                    ctx,
                    &font_sheet.data.chars,
                    font_sheet.data.width,
                    font_sheet.data.height,
                    &font_sheet.data.custom,
                    Image::new(&font_sheet.sheet)?,
                )?,
            },
        );
        Ok(())
    }

    pub fn draw_text_left(
        &self,
        ctx: &mut Context,
        font: &FontId,
        text: &str,
        x: f32,
        y: f32,
        params: DrawParams,
    ) {
        if let Some(font) = self.fonts.get(font) {
            font.draw_text_left(ctx, text, x, y, params);
        }
    }

    pub fn draw_text_right(
        &self,
        ctx: &mut Context,
        font: &FontId,
        text: &str,
        x: f32,
        y: f32,
        params: DrawParams,
    ) {
        if let Some(font) = self.fonts.get(font) {
            font.draw_text_right(ctx, text, x, y, params);
        }
    }

    pub fn draw_text_center(
        &self,
        ctx: &mut Context,
        font: &FontId,
        text: &str,
        center_vertical: bool,
        x: f32,
        y: f32,
        params: DrawParams,
    ) {
        if let Some(font) = self.fonts.get(font) {
            font.draw_text_center(ctx, text, center_vertical, x, y, params);
        }
    }

    pub fn draw_button_for_text(
        &self,
        ctx: &mut Context,
        font: &FontId,
        text: &str,
        x: f32,
        y: f32,
        params: DrawParams,
    ) {
        if let Some(font) = self.fonts.get(font) {
            self.draw_button(
                ctx,
                x + font.text_pixel_length(text) as f32,
                y + 2.0,
                params,
            )
        }
    }

    pub fn draw_button(&self, ctx: &mut Context, x: f32, y: f32, params: DrawParams) {
        self.button.draw(ctx, x, y, params);
    }

    pub fn draw_cursor(&self, ctx: &mut Context, x: f32, y: f32, params: DrawParams) {
        self.cursor.draw(ctx, x, y, params);
    }

    pub fn text_len(&self, font: &FontId, text: &str) -> f32 {
        if let Some(font) = self.fonts.get(font) {
            font.text_pixel_length(text)
        } else {
            0.0
        }
    }
}

pub(crate) fn iterate_fontsheet(
    ctx: &mut Context,
    chars: &str,
    font_width: FontDimensions,
    font_height: FontDimensions,
    custom: &[CustomChar],
    sheet: Image,
) -> Result<CharTextures, ImageError> {
    let mut customchars: HashMap<char, (FontDimensions, Option<FontDimensions>)> = custom
        .into_iter()
        .map(|cchar| (cchar.id, (cchar.width, cchar.height)))
        .collect();

    let chars: Vec<char> = chars.chars().collect();
    let sheet_width = sheet.width();
    let sheet_height = sheet.height(); // - font_height as u32;

    let mut charmap = HashMap::with_capacity(chars.len());

    let mut counter: usize = 0;
    let mut x = 0;
    let mut y = 0;

    'yloop: while y < sheet_height {
        while x < sheet_width {
            charmap.insert(
                chars[counter],
                if let Some(cchar) = customchars.remove(&chars[counter]) {
                    Texture::from_image(
                        ctx,
                        &Image::from(sheet.region(
                            x,
                            y,
                            cchar.0 as _,
                            cchar.1.unwrap_or(font_height) as _,
                        )),
                    )
                } else {
                    Texture::from_image(
                        ctx,
                        &Image::from(sheet.region(x, y, font_width as _, font_height as _)),
                    )
                },
            );
            x += font_width as u32;
            counter += 1;
            if counter >= chars.len() {
                break 'yloop;
            }
        }
        x = 0;
        y += font_height as u32;
    }

    Ok(charmap)
}
