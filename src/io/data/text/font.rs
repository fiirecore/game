use macroquad::prelude::warn;
use serde::Deserialize;
use ahash::AHashMap as HashMap;

use crate::util::text::font::Font;
use crate::util::graphics::texture::image_texture;
#[derive(Debug, Deserialize)]
pub struct FontSheetData {

    pub file: String,
    pub width: u8,
    pub height: u8,
    pub chars: String,
    pub custom: Vec<CustomChars>,

}

#[derive(Debug, Deserialize)]
pub struct CustomChars {

    pub id: char,
    pub width: u8,
    pub height: Option<u8>,

}

impl FontSheetData {

    pub fn open_sheet<P: AsRef<std::path::Path>>(path: P) -> Option<Font> {
        let path = path.as_ref();
        match crate::io::get_file(path) {
            Some(file) => match serde_json::from_slice(&*file) {
                Ok(sheet) => return Self::sheet_image(sheet),
                Err(err) => {
                    warn!("Could not parse font sheet data with error {}", err);
                    return None;
                }
            },
            None => {
                warn!("Could not open font sheet config at {:?}", path);
                return None;
            }
        }

    }
    
    pub fn into_sheet(self, sheet: macroquad::prelude::Image) -> Font {
        Font {
            font_width: self.width,
            font_height: self.height,
            chars: iterate_fontsheet(self.chars, self.width, self.height, self.custom, sheet),
        }        
    }

    fn sheet_image(self) -> Option<Font> {
        match crate::io::get_file(std::path::PathBuf::from("fonts").join(&self.file)) {
            Some(ref file) => match crate::util::image::byte_image(file) {
                Ok(image) => Some(self.into_sheet(image)),
                Err(err) => {
                    warn!("Could not parse font sheet at {} with error {}", &self.file, err);
                    return None;
                }
            },
            None => {
                warn!("Could not open font image at {}", &self.file);
                return None;
            },
        }
    }

}

fn iterate_fontsheet(chars: String, font_width: u8, font_height: u8, custom: Vec<CustomChars>, sheet: macroquad::prelude::Image) -> HashMap<char, crate::util::graphics::Texture> {

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
                charmap.insert(chars[counter], image_texture(&sheet.get_subimage(x, y, cchar.0 as u32, cchar.1.unwrap_or(font_height) as u32)));
            } else {
                charmap.insert(chars[counter], image_texture(&sheet.get_subimage(x, y, font_width as u32, font_height as u32)));
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