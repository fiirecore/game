use firecore_dependencies::{
    hash::HashMap,
};

use macroquad::prelude::{Color, color_u8, Image, Texture2D, draw_texture, WHITE, Rect, FilterMode::Nearest};

use firecore_font::{SerializedFonts, CustomChar, message::TextColor};

pub use firecore_font::{FontId, default_font_id, message};

pub static mut TEXT_RENDERER: Option<TextRenderer> = None;

pub struct TextRenderer {

    pub fonts: HashMap<FontId, Font>,
    pub button: Texture2D,
    pub cursor: Texture2D,

}

impl TextRenderer {

    pub fn new() -> TextRenderer {
        TextRenderer {
            fonts: HashMap::new(),
            button: {
                let texture = Texture2D::from_file_with_format(include_bytes!("../assets/button.png"), None);
                texture.set_filter(Nearest);
                texture
            },
            cursor: {
                let texture = Texture2D::from_file_with_format(include_bytes!("../assets/cursor.png"), None);
                texture.set_filter(Nearest);
                texture
            },
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
            draw_texture(self.button, x + font.text_pixel_length(text) as f32, y + 2.0, WHITE);
        }
    }

    pub fn render_cursor(&self, x: f32, y: f32) {
        draw_texture(self.cursor, x, y, WHITE);
    }

}

pub struct Font {

    pub width: u8,
    pub height: u8,

    pub chars: HashMap<char, Texture2D>,

}

impl Font {

    pub fn render_text_from_left(&self, text: &str, x: f32, y: f32, color: Color) {
        let mut len: u32 = 0;
        for character in text.chars() {
            len += if let Some(texture) = self.chars.get(&character) {
                draw_texture(*texture, x + len as f32, y, color);
                texture.width() as u32
            } else {
                self.width as u32
            };       
        }
    }

    pub fn render_text_from_right(&self, text: &str, x: f32, y: f32, color: Color) {
        let mut len = 0.0;
        let x = x - self.text_pixel_length(text);
        for character in text.chars() {
            len += if let Some(texture) = self.chars.get(&character) {
                draw_texture(*texture, x + len, y, color);
                texture.width()
            } else {
                self.width as f32
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
                self.width as f32
            };
        }
    }

    pub fn text_pixel_length(&self, text: &str) -> f32 {
        text.chars().map(|character| {
            match self.chars.get(&character) {
                Some(texture) => texture.width(),
                None => self.width as f32,
            }
        }).sum()
    }

}

pub trait IntoMQColor: Copy {

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

pub fn init(font_sheets: SerializedFonts) {
	let mut text_renderer = TextRenderer::new();

    for font_sheet in font_sheets.fonts {
        text_renderer.fonts.insert(
            font_sheet.data.id, 
            Font {
                width: font_sheet.data.width,
                height: font_sheet.data.height,
                chars: iterate_fontsheet(
                    font_sheet.data.chars, 
                    font_sheet.data.width, 
                    font_sheet.data.height, 
                    font_sheet.data.custom, 
                    Image::from_file_with_format(&font_sheet.image, None)
                ),
            }
        );
    }

	unsafe { TEXT_RENDERER = Some(text_renderer); }
}

fn iterate_fontsheet(chars: String, font_width: u8, font_height: u8, custom: Vec<CustomChar>, sheet: Image) -> HashMap<char, Texture2D> {

    let mut customchars: HashMap<char, (u8, Option<u8>)> = custom.into_iter().map(|cchar| (cchar.id, (cchar.width, cchar.height))).collect();

    let chars: Vec<char> = chars.chars().collect();
    let sheet_width = sheet.width() as f32;
    let sheet_height = sheet.height() as f32;// - font_height as u32;

    let mut charmap = HashMap::with_capacity(chars.len());

    let mut counter: usize = 0;
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;

    'yloop: while y < sheet_height {
        while x < sheet_width {
            if let Some(cchar) = customchars.remove(&chars[counter]) {
                charmap.insert(chars[counter], {
                    let texture = Texture2D::from_image(&sheet.sub_image(Rect::new(x, y, cchar.0 as f32, cchar.1.unwrap_or(font_height) as f32)));
                    texture.set_filter(Nearest);
	                texture
                });
            } else {
                charmap.insert(chars[counter], {
                    let texture = Texture2D::from_image(&sheet.sub_image(Rect::new(x, y, font_width as f32, font_height as f32)));
                    texture.set_filter(Nearest);
	                texture
                });
            }
            x += font_width as f32;
            counter+=1;
            if counter >= chars.len() {
                break 'yloop;
            }
        }
        x = 0.0;
        y += font_height as f32;
    }

    charmap
}