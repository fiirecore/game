use std::collections::HashMap;
use opengl_graphics::{GlGraphics, ImageSize, Texture, TextureSettings, Filter};
use piston_window::Context;

use image::RgbaImage;

use crate::util::render_util::{draw, draw_o};
use crate::util::{file::asset_as_pathbuf, texture_util::texture_from_path};
use crate::util::image_util::{open_image, get_subimage_wh, get_subimage_at};

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
        draw_o(ctx, g, self.button.as_ref(), x + len, y + offset as isize + 5);
    }

    pub fn render_cursor(&self, ctx: &mut Context, g: &mut GlGraphics, x: isize, y: isize) {
        draw_o(ctx, g, self.cursor.as_ref(), x, y);
    }

    pub fn load_textures(&mut self) {
        self.button = Some(texture_from_path(asset_as_pathbuf("gui/button.png")));
        self.cursor = Some(texture_from_path(asset_as_pathbuf("gui/cursor.png")));
    }

    pub fn default_add(&mut self) {
        self.default_add_type0();
        self.default_add_type1(1);
        self.default_add_type1(2);
    }

    fn default_add_type0(&mut self) {
        self.fonts.push(HashMap::new());
        let font_sheet0 = open_image(asset_as_pathbuf("font0.png")).expect("Could not find font sheet #0!");
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
        self.fonts[0].insert('I', get_tex(&get_subimage_at(&font_sheet0, 40, 0, 4, TEXT_HEIGHT0 as u32)));
        self.fonts[0].insert('T', get_tex(&get_subimage_at(&font_sheet0, 95, 0, 4, TEXT_HEIGHT0 as u32)));
        self.fonts[0].insert('Y', get_tex(&get_subimage_at(&font_sheet0, 120, 0, 4, TEXT_HEIGHT0 as u32)));
        self.fonts[0].insert('i', get_tex(&get_subimage_at(&font_sheet0, 40, 12, 4, TEXT_HEIGHT0 as u32)));
    }

    fn default_add_type1(&mut self, id: usize) {
        let mut filename = String::from("font");
        filename.push_str(id.to_string().as_str());
        filename.push_str(".png");
        self.fonts.push(HashMap::new());
        let font_sheet1 = open_image(asset_as_pathbuf(filename)).expect("Could not find font sheet #1!");
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
        self.fonts[id].insert('i', get_tex(&get_subimage_at(&font_sheet1, 48, 14, 4, 14)));
        self.fonts[id].insert('k', get_tex(&get_subimage_at(&font_sheet1, 60, 14, 5, 14)));
        self.fonts[id].insert('l', get_tex(&get_subimage_at(&font_sheet1, 66, 14, 5, 14)));
        self.fonts[id].insert('n', get_tex(&get_subimage_at(&font_sheet1, 78, 14, 5, 14)));
        self.fonts[id].insert('r', get_tex(&get_subimage_at(&font_sheet1, 102, 14, 5, 14)));
        self.fonts[id].insert('s', get_tex(&get_subimage_at(&font_sheet1, 108, 14, 5, 14)));
        self.fonts[id].insert('t', get_tex(&get_subimage_at(&font_sheet1, 114, 14, 5, 14)));
    }

    pub(crate) fn add_char_from_sheet(&mut self, character: char, font_id: usize, font_sheet: &RgbaImage, index: usize, width: u8, height: u8) {
        self.fonts[font_id].insert(character, get_tex(&get_subimage_wh(&font_sheet, index, width as u32, height as u32)));
    }

    pub(crate) fn remove_char(&mut self, font_id: usize, character: char) {
        self.fonts[font_id].remove(&character);
    }

}

fn get_tex(image: &RgbaImage) -> Texture {
    //return Image::from_rgba8(ctx, image.width() as u16, image.height() as u16, image.to_vec().as_mut_slice()).expect("Could not get texture for text");
    return Texture::from_image(image, &TextureSettings::new().min(Filter::Nearest).mag(Filter::Nearest));
}