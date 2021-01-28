use core::f32;

use ahash::AHashMap;
use macroquad::prelude::Image;

use crate::util::texture::Texture;

use crate::util::render::draw;
use crate::util::image::{get_subimage_wh, get_subimage_at};

use super::image::open_image_bytes;
use super::texture::byte_texture;
use super::texture::image_texture;

pub struct TextRenderer {

    pub(crate) fonts: Vec<AHashMap<char, Texture>>,
    pub(crate) button: Texture,
    pub(crate) cursor: Texture,

}

pub static TEXT_WIDTH0: u8 = 5;
pub static TEXT_HEIGHT0: u8 = 12;

pub static TEXT_WIDTH1: u8 = 6;
pub static TEXT_HEIGHT1: u8 = 14;

impl TextRenderer {

    pub fn new() -> TextRenderer {
        TextRenderer {
            fonts: Vec::new(),
            button: byte_texture(include_bytes!("../../include/gui/button.png")),
            cursor: byte_texture(include_bytes!("../../include/gui/cursor.png")),
        }
    }

    pub fn render_text_from_left(&self, font_id: usize, text: &str, x: f32, y: f32) {
        let mut len: u32 = 0;
        for character in text.chars() {
            len += match self.fonts[font_id].get(&character) {
                Some(texture) => {
                    draw(*texture, x + len as f32, y);
                    texture.width() as u32
                },
                None => {
                    self.fonts[font_id].values().next().unwrap().width() as u32
                }
            };          
        }
    }

    pub fn render_text_from_right(&self, font_id: usize, text: &str, x: f32, y: f32) {
        let mut len = 0.0;
        let x_offset = self.text_pixel_length(font_id, text) as f32;
        for character in text.chars() {
            len += match self.fonts[font_id].get(&character) {
                Some(texture) => {
                    draw(*texture, x - x_offset + len as f32, y);
                    texture.width()
                },
                None => {
                    self.fonts[font_id].values().next().unwrap().width()
                }
            };         
        }
    }

    pub fn text_pixel_length(&self, font_id: usize, text: &str) -> f32 {
        let mut len = 0.0;
        for character in text.chars() {
            len += if let Some(texture) = self.fonts[font_id].get(&character) {
                texture.width()
            } else {
                self.fonts[font_id].values().next().unwrap().width()
            }
        };
        return len;
    }

    pub fn render_button(&self, text: &str, font_id: usize, offset: f32, x: f32, y: f32) {
        draw(self.button, x + self.text_pixel_length(font_id, text) as f32, y + offset + 5.0);
    }

    pub fn render_cursor(&self, x: f32, y: f32) {
        draw(self.cursor, x, y);
    }

    pub fn default_add(&mut self) {
        self.default_add_type0();
        self.default_add_type1(include_bytes!("../../include/font1.png"), 1);
        self.default_add_type1(include_bytes!("../../include/font2.png"), 2);
    }

    fn default_add_type0(&mut self) {
        self.fonts.push(AHashMap::new());
        let font_sheet0 = open_image_bytes(include_bytes!("../../include/font0.png"));
        let alphanumerics = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789/♂♀".chars();
        let mut index: usize = 0;
        for character in alphanumerics {
            self.add_char_from_sheet(character, 0, &font_sheet0, index, TEXT_WIDTH0, TEXT_HEIGHT0);            
            index+=1;
        }
        self.remove_char(0, 'I');
        self.remove_char(0, 'T');
        self.remove_char(0, 'Y');
        self.remove_char(0, 'i');
        self.fonts[0].insert('I', image_texture(&get_subimage_at(&font_sheet0, 40, 0, 4, TEXT_HEIGHT0 as u32)));
        self.fonts[0].insert('T', image_texture(&get_subimage_at(&font_sheet0, 95, 0, 4, TEXT_HEIGHT0 as u32)));
        self.fonts[0].insert('Y', image_texture(&get_subimage_at(&font_sheet0, 120, 0, 4, TEXT_HEIGHT0 as u32)));
        self.fonts[0].insert('i', image_texture(&get_subimage_at(&font_sheet0, 40, 12, 4, TEXT_HEIGHT0 as u32)));
    }

    fn default_add_type1(&mut self, bytes: &[u8], id: usize) {
        let font_sheet1 = open_image_bytes(bytes);
        self.fonts.push(AHashMap::new());
        
        let alphanumerics = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789/♂♀!?".chars();
        let mut index: usize = 0;
        for character in alphanumerics {
            self.add_char_from_sheet(character, id, &font_sheet1, index, TEXT_WIDTH1, TEXT_HEIGHT1);            
            index+=1;
        }
        self.remove_char(id, 'i');
        self.remove_char(id, 'k');
        self.remove_char(id, 'l');
        self.remove_char(id, 'n');
        self.remove_char(id, 'r');
        self.remove_char(id, 's');
        self.remove_char(id, 't');
        self.fonts[id].insert('i', image_texture(&get_subimage_at(&font_sheet1, 48, 14, 4, 14)));
        self.fonts[id].insert('k', image_texture(&get_subimage_at(&font_sheet1, 60, 14, 5, 14)));
        self.fonts[id].insert('l', image_texture(&get_subimage_at(&font_sheet1, 66, 14, 5, 14)));
        self.fonts[id].insert('n', image_texture(&get_subimage_at(&font_sheet1, 78, 14, 5, 14)));
        self.fonts[id].insert('r', image_texture(&get_subimage_at(&font_sheet1, 102, 14, 5, 14)));
        self.fonts[id].insert('s', image_texture(&get_subimage_at(&font_sheet1, 108, 14, 5, 14)));
        self.fonts[id].insert('t', image_texture(&get_subimage_at(&font_sheet1, 114, 14, 5, 14)));
    }

    pub(crate) fn add_char_from_sheet(&mut self, character: char, font_id: usize, font_sheet: &Image, index: usize, width: u8, height: u8) {
        self.fonts[font_id].insert(character, image_texture(&get_subimage_wh(&font_sheet, index, width as u32, height as u32)));
    }

    pub(crate) fn remove_char(&mut self, font_id: usize, character: char) {
        self.fonts[font_id].remove(&character);
    }

}