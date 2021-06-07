use firecore_dependencies::tetra::graphics::DrawParams;
use firecore_dependencies::tetra::math::Vec2;
use firecore_dependencies::{
    hash::HashMap,
    tetra::{
        Result,
        Context,
        graphics::{
            Texture,
            ImageData,
            Rectangle,
            Color,
        },
    },
};

use firecore_font::{SerializedFonts, CustomChar, message::TextColor};

pub use firecore_font::{FontId, default_font_id, message};

pub static mut TEXT_RENDERER: Option<TextRenderer> = None;

pub struct TextRenderer {

    pub fonts: HashMap<FontId, Font>,
    pub button: Texture,
    pub cursor: Texture,

}

impl TextRenderer {

    pub fn new(ctx: &mut Context) -> Result<TextRenderer> {
        Ok(TextRenderer {
            fonts: HashMap::new(),
            button: Texture::from_file_data(ctx, include_bytes!("../assets/button.png"))?,
            cursor: Texture::from_file_data(ctx, include_bytes!("../assets/cursor.png"))?,
        })
    }

    pub fn draw_text_left(&self, ctx: &mut Context, font: &FontId, text: &str, color: &Color, x: f32, y: f32) {
        if let Some(font) = self.fonts.get(font) {
            font.draw_text_left(ctx, text, x, y, color);
        }
    }

    pub fn draw_text_right(&self, ctx: &mut Context, font: &FontId, text: &str, color: &Color, x: f32, y: f32) { // To - do: Have struct that stores a message, font id and color
        if let Some(font) = self.fonts.get(font) {
            font.draw_text_right(ctx, text, x, y, color);
        }
    }

    pub fn draw_text_center(&self, ctx: &mut Context, font: &FontId, text: &str, color: &Color, x: f32, y: f32) { // To - do: Have struct that stores a message, font id and color
        if let Some(font) = self.fonts.get(font) {
            font.draw_text_center(ctx, text, x, y, color);
        }
    }

    pub fn draw_button(&self, ctx: &mut Context, font: &FontId, text: &str, x: f32, y: f32) {
        if let Some(font) = self.fonts.get(font) {
            self.button.draw(ctx, DrawParams::position(DrawParams::default(), Vec2::new(x + font.text_pixel_length(text) as f32, y + 2.0)));
        }
    }

    pub fn draw_cursor(&self, ctx: &mut Context, x: f32, y: f32) {
        self.cursor.draw(ctx, DrawParams::position(DrawParams::default(), Vec2::new(x, y)));
    }

    pub fn text_len(&self, font: &FontId, text: &str) -> f32 {
        if let Some(font) = self.fonts.get(font) {
            font.text_pixel_length(text)
        } else {
            0.0
        }
    }

}

pub struct Font {

    pub width: u8,
    pub height: u8,

    pub chars: HashMap<char, Texture>,

}

impl Font {

    pub fn draw_text_left(&self, ctx: &mut Context, text: &str, x: f32, y: f32, color: &Color) {
        let mut len = 0;
        for character in text.chars() {
            len += if let Some(texture) = self.chars.get(&character) {
                texture.draw(ctx, DrawParams::position(DrawParams::default(), Vec2::new(x + len as f32, y)).color(*color));
                texture.width()
            } else {
                self.width as _
            };       
        }
    }

    pub fn draw_text_right(&self, ctx: &mut Context, text: &str, x: f32, y: f32, color: &Color) {
        let mut len = 0;
        let x = x - self.text_pixel_length(text);
        for character in text.chars() {
            len += if let Some(texture) = self.chars.get(&character) {
                texture.draw(ctx, DrawParams::position(DrawParams::default(), Vec2::new(x + len as f32, y)).color(*color));
                texture.width()
            } else {
                self.width as _
            };
        }
    }

    pub fn draw_text_center(&self, ctx: &mut Context, text: &str, x: f32, y: f32, color: &Color) {
        let mut len = 0;
        let x_offset = self.text_pixel_length(text) / 2.0;
        for character in text.chars() {
            len += if let Some(texture) = self.chars.get(&character) {
                texture.draw(ctx, DrawParams::position(DrawParams::default(), Vec2::new(x - x_offset + len as f32, y)).color(*color));
                texture.width()
            } else {
                self.width as _
            };
        }
    }

    pub fn text_pixel_length(&self, text: &str) -> f32 {
        text.chars().map(|character| {
            match self.chars.get(&character) {
                Some(texture) => texture.width() as f32,
                None => self.width as f32,
            }
        }).sum()
    }

}

pub trait AsColor {
    fn as_color(&self) -> &Color;
}

impl AsColor for TextColor {
    fn as_color(&self) -> &Color {
        match self {
            TextColor::White => WHITE_COLOR,
            TextColor::Gray => GRAY,
            TextColor::Black => BLACK_COLOR,
            TextColor::Red => RED,
            TextColor::Blue => BLUE_COLOR,
        }
    }
}

const GRAY: &Color = &Color::rgb(0.51, 0.51, 0.51);
const RED: &Color = &Color::rgb(0.90, 0.16, 0.22);
const WHITE_COLOR: &Color = &Color::rgb(240.0 / 255.0, 240.0 / 255.0, 240.0 / 255.0);
const BLACK_COLOR: &Color = &Color::rgb(20.0 / 255.0, 20.0 / 255.0, 20.0 / 255.0);
const BLUE_COLOR: &Color = &Color::rgb(48.0 / 255.0, 80.0 / 255.0, 200.0 / 255.0);

pub fn init(ctx: &mut Context, font_sheets: SerializedFonts) -> Result {
	let mut text_renderer = TextRenderer::new(ctx)?;

    for font_sheet in font_sheets.fonts {
        text_renderer.fonts.insert(
            font_sheet.data.id, 
            Font {
                width: font_sheet.data.width,
                height: font_sheet.data.height,
                chars: iterate_fontsheet(
                    ctx,
                    font_sheet.data.chars, 
                    font_sheet.data.width, 
                    font_sheet.data.height, 
                    font_sheet.data.custom, 
                    ImageData::from_file_data(&font_sheet.image)?
                )?,
            }
        );
    }

	unsafe { TEXT_RENDERER = Some(text_renderer); }

    Ok(())
}

fn iterate_fontsheet(ctx: &mut Context, chars: String, font_width: u8, font_height: u8, custom: Vec<CustomChar>, sheet: ImageData) -> Result<HashMap<char, Texture>> {

    let mut customchars: HashMap<char, (u8, Option<u8>)> = custom.into_iter().map(|cchar| (cchar.id, (cchar.width, cchar.height))).collect();

    let chars: Vec<char> = chars.chars().collect();
    let sheet_width = sheet.width() as _;
    let sheet_height = sheet.height() as _;// - font_height as u32;

    let mut charmap = HashMap::with_capacity(chars.len());

    let mut counter: usize = 0;
    let mut x = 0;
    let mut y = 0;

    'yloop: while y < sheet_height {
        while x < sheet_width {
            charmap.insert(chars[counter], if let Some(cchar) = customchars.remove(&chars[counter]) {
                Texture::from_image_data(ctx, &sheet.region(Rectangle::new(x, y, cchar.0 as _, cchar.1.unwrap_or(font_height) as _)))
            } else {
                Texture::from_image_data(ctx, &sheet.region(Rectangle::new(x, y, font_width as _, font_height as _)))
            }?);
            x += font_width as i32;
            counter+=1;
            if counter >= chars.len() {
                break 'yloop;
            }
        }
        x = 0;
        y += font_height as i32;
    }

    Ok(charmap)
}