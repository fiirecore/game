use firecore_font_lib::CustomChar;
use macroquad::prelude::Rect;
use macroquad::prelude::warn;
use ahash::AHashMap as HashMap;

use crate::util::text::font::Font;
use crate::util::graphics::texture::image_texture;

pub async fn open_sheets() {

    let font_sheets: firecore_font_lib::SerializedFonts = bincode::deserialize(
        // &macroquad::prelude::load_file("assets/fonts.bin").await.unwrap()
        include_bytes!("../../../assets/fonts.bin")
    ).unwrap();

    for font_sheet in font_sheets.fonts {
        match crate::util::image::byte_image(&font_sheet.image) {
            Ok(image) => {
                crate::util::text::FONTS.insert(
                    font_sheet.data.id, 
                    Font {
                        font_width: font_sheet.data.width,
                        font_height: font_sheet.data.height,
                        chars: iterate_fontsheet(font_sheet.data.chars, font_sheet.data.width, font_sheet.data.height, font_sheet.data.custom, image),
                    }
                );
            },
            Err(err) => {
                warn!("Could not parse font sheet {}'s image with error {}", &font_sheet.data.id, err);
            }
        }
    }
}

fn iterate_fontsheet(chars: String, font_width: u8, font_height: u8, custom: Vec<CustomChar>, sheet: macroquad::prelude::Image) -> HashMap<char, crate::util::graphics::Texture> {

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