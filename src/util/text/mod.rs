use dashmap::DashMap;
use firecore_util::text::MessageSet;
use macroquad::prelude::Color;

use crate::data::player::list::PlayerSaves;
use crate::util::graphics::Texture;
use crate::util::graphics::draw;

use self::font::Font;

use super::graphics::texture::byte_texture;

pub mod font;

lazy_static::lazy_static! {
    pub static ref FONTS: DashMap<usize, Font> = DashMap::new();
}

pub fn process_message_set(message_set: &mut MessageSet) {
    for message in &mut message_set.messages {
        for message in &mut message.message {
            *message = message
                .replace("%r", rival_name())
                .replace("%p", &player_name())
                
            ;
        }
    }
}

pub fn player_name() -> String {
    if let Some(player_saves) = macroquad::prelude::collections::storage::get::<PlayerSaves>() {
        player_saves.get().name.clone()
    } else {
        crate::data::player::save::default_name()
    }
}

pub fn rival_name() -> &'static str {
    "Gary"
}

pub struct TextRenderer {

    // pub fonts: [Font; 3],
    pub button: Texture,
    pub cursor: Texture,

}

impl TextRenderer {

    pub fn new() -> TextRenderer {
        TextRenderer {
            button: byte_texture(include_bytes!("../../../build/assets/gui/button.png")),
            cursor: byte_texture(include_bytes!("../../../build/assets/gui/cursor.png")),
        }
    }

    pub fn render_text_from_left(&self, font_id: usize, text: &str, color: Color, x: f32, y: f32) {
        if let Some(font) = FONTS.get(&font_id) {
            font.render_text_from_left(text, x, y, color);
        }
    }

    pub fn render_text_from_right(&self, font_id: usize, text: &str, color: Color, x: f32, y: f32) { // To - do: Have struct that stores a message, font id and color
        if let Some(font) = FONTS.get(&font_id) {
            font.render_text_from_right(text, x, y, color);
        }
    }

    pub fn render_button(&self, text: &str, font_id: usize, x: f32, y: f32) {
        if let Some(font) = FONTS.get(&font_id) {
            draw(self.button, x + font.text_pixel_length(text) as f32, y + 2.0);
        }
    }

    pub fn render_cursor(&self, x: f32, y: f32) {
        draw(self.cursor, x, y);
    }

}