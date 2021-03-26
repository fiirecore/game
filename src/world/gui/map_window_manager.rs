use firecore_util::{Entity, Reset, Completable, text::Message};
use macroquad::prelude::Vec2;

use crate::gui::dynamic_text::DynamicText;
use crate::util::text::process_messages;

use crate::util::graphics::{Texture, draw};

pub struct MapWindowManager {

    alive: bool,
    pos: Vec2,
    background: Texture,
    text: DynamicText,

}

impl MapWindowManager {

    pub fn reset_text(&mut self) {
        self.text.reset();
    }

    pub fn set_text(&mut self, messages: Vec<Message>) {
        self.text.messages = Some(messages);
    }

    pub fn on_start(&mut self) {
        if let Some(messages) = self.text.messages.as_mut() {
            process_messages(messages);
        }
        
    }

    pub fn update(&mut self, delta: f32) {
        if self.is_alive() {
            self.text.update(delta);
        }
    }

    pub fn render(&self) {
        if self.is_alive() {
            draw(self.background, self.pos.x, self.pos.y);
            self.text.render();
        }
    }

    pub fn input(&mut self) {
        self.text.input();
    }

}

impl Default for MapWindowManager {
    fn default() -> Self {
        let pos = Vec2::new(6.0, 116.0);
        MapWindowManager {
            alive: false,
            pos,
            background: crate::util::graphics::texture::byte_texture(include_bytes!("../../../build/assets/gui/message.png")),
            text: DynamicText::new(Vec2::new(11.0, 5.0), pos),
        }
    }
}

impl Entity for MapWindowManager {
    fn spawn(&mut self) {
        self.alive = true;
        self.reset();
        self.text.spawn();
    }

    fn despawn(&mut self) {
        self.alive = false;
        self.text.despawn();
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}

impl Reset for MapWindowManager {
    fn reset(&mut self) {
        self.text.reset();
    }
}

impl Completable for MapWindowManager {

    fn is_finished(&self) -> bool {
        self.text.is_finished()
    }

}