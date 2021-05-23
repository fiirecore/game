use crate::{
    util::Entity,
    gui::DynamicText,
    text::TextColor,
    graphics::{byte_texture, draw},
    macroquad::prelude::{Vec2, const_vec2, Texture2D}
};

pub struct TextWindow {

    background: Texture2D,
    pub text: DynamicText,

}

impl TextWindow {

    const ORIGIN: Vec2 = const_vec2!([6.0, 116.0]);
    const TEXT_OFFSET: Vec2 = const_vec2!([11.0, 5.0]);

    pub fn render(&self) {
        if self.text.alive() {
            draw(self.background, Self::ORIGIN.x, Self::ORIGIN.y);
            self.text.render();
        }
    }

}

impl Default for TextWindow {
    fn default() -> Self {
        Self {
            background: byte_texture(include_bytes!("../../../assets/world/gui/message.png")),
            text: DynamicText::new(Self::ORIGIN + Self::TEXT_OFFSET, 1, TextColor::Black, 5),
        }
    }
}