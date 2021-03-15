use std::hash::Hash;
use std::hash::Hasher;

use firecore_util::text::MessageSet;

use firecore_util::Entity;
use crate::gui::Focus;
use crate::gui::GuiComponent;
use crate::gui::background::Background;
use crate::gui::dynamic_text::DynamicText;
use crate::util::Completable;
use crate::util::Input;
use crate::util::Reset;
use crate::util::text::process_message_set;

pub struct MapWindowManager {

    alive: bool,
    background: Background,
    text: DynamicText,

}

impl MapWindowManager {

    pub fn new() -> MapWindowManager {
        let panel_x = 6.0;
        let panel_y = 116.0;
        MapWindowManager {
            alive: false,
            background: Background::default(),
            text: DynamicText::new(11.0, 5.0, panel_x, panel_y),
        }
    }

    pub fn set_text(&mut self, message_set: MessageSet) {
        self.text.text = message_set;
        process_message_set(&mut self.text.text);
    }

    pub fn text_hash(&self) -> u64 {
        let mut hasher = ahash::AHasher::default();
        self.text.text.hash(&mut hasher);
        hasher.finish()
    }

    pub fn update(&mut self, delta: f32) {
        if self.is_alive() {
            self.text.update(delta);
        }
    }

    pub fn render(&self) {
        if self.is_alive() {
            self.background.render();
            self.text.render();
        }
    }

    pub fn input(&mut self, delta: f32) {
        self.text.input(delta);
    }

}

impl Default for MapWindowManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Entity for MapWindowManager {
    fn spawn(&mut self) {
        self.alive = true;
        self.reset();
        self.text.spawn();
        self.text.focus();
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