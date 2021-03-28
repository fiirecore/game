use firecore_data::player::save::PlayerSave;
use firecore_util::text::Message;
use renderer::TextRenderer;

use firecore_util::text::TextColor;
use macroquad::prelude::{Color, color_u8};

use self::font::Font;

pub mod renderer;
pub mod font;

pub static mut TEXT_RENDERER: Option<TextRenderer> = None;

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
const BLUE_COLOR: Color = color_u8!(48, 80, 200, 255); // 48, 80, 200

pub async fn init_text() {
	let mut text_renderer = TextRenderer::new();

	let font_sheets: firecore_font_lib::SerializedFonts = bincode::deserialize(
        // &macroquad::prelude::load_file("assets/fonts.bin").await.unwrap()
        include_bytes!("../../../assets/fonts.bin")
    ).unwrap();

    for font_sheet in font_sheets.fonts {
        text_renderer.fonts.insert(
            font_sheet.data.id, 
            Font {
                font_width: font_sheet.data.width,
                font_height: font_sheet.data.height,
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

// #[deprecated(note = "make better")]
pub fn process_messages(player_save: &PlayerSave, messages: &mut Vec<Message>) {
    for message in messages.iter_mut() {
        for message in &mut message.lines {
            *message = message
                .replace("%r", rival_name())
                .replace("%p", player_name(player_save))
            ;
        }
    }
}

pub fn player_name(player_save: &PlayerSave) -> &String {
    &player_save.name
}

pub fn rival_name() -> &'static str {
    "Gary"
}

use firecore_font_lib::CustomChar;
use macroquad::prelude::{Image, Rect, Texture2D};
use ahash::AHashMap as HashMap;

use crate::util::graphics::image_texture;

pub fn iterate_fontsheet(chars: String, font_width: u8, font_height: u8, custom: Vec<CustomChar>, sheet: Image) -> HashMap<char, Texture2D> {

    let mut customchars = HashMap::new();
    for cchar in custom {
        customchars.insert(cchar.id, (cchar.width, cchar.height));
    }

    let chars: Vec<char> = chars.chars().collect();
    let sheet_width = sheet.width() as u32;
    let sheet_height = sheet.height() as u32;// - font_height as u32;

    let mut charmap = HashMap::new();

    let mut counter: usize = 0;
    let mut x: u32 = 0;
    let mut y: u32 = 0;

    while y < sheet_height {
        while x < sheet_width {
            if let Some(cchar) = customchars.remove(&chars[counter]) {
                charmap.insert(chars[counter], image_texture(&sheet.sub_image(Rect::new(x as f32, y as f32, cchar.0 as f32, cchar.1.unwrap_or(font_height) as f32))));
            } else {
                charmap.insert(chars[counter], image_texture(&sheet.sub_image(Rect::new(x as f32, y as f32, font_width as f32, font_height as f32))));
            }
            x += font_width as u32;
            counter+=1;
            if counter >= chars.len() {
                return charmap;
            }
        }
        x = 0;
        y += font_height as u32;
    }

    return charmap;
}