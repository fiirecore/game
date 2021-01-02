use std::collections::HashMap;

use image::RgbaImage;
use opengl_graphics::Filter;
use opengl_graphics::Texture;
use opengl_graphics::TextureSettings;

use crate::util::{file_util::asset_as_pathbuf, texture_util::texture_from_path};
use crate::util::image_util::{open_image, get_subimage_wh, get_subimage_at};

use super::text::TextRenderer;
use super::text::{TEXT_WIDTH0, TEXT_WIDTH1, TEXT_HEIGHT0, TEXT_HEIGHT1};

impl TextRenderer {

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