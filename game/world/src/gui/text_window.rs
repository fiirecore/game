use firecore_game::util::{Entity, Reset, Completable, text::Message};
use firecore_game::macroquad::prelude::{Vec2, Texture2D};
use firecore_game::data::player::PlayerSave;

use firecore_game::gui::text::DynamicText;
use firecore_game::text::process_messages;
use firecore_game::graphics::{byte_texture, draw};

pub struct TextWindow {

    alive: bool,
    pos: Vec2,
    background: Texture2D,
    text: DynamicText,

}

impl TextWindow {

    pub fn reset_text(&mut self) {
        self.text.reset();
    }

    pub fn set_text(&mut self, messages: Vec<Message>) {
        self.text.messages = Some(messages);
    }

    pub fn on_start(&mut self, player_save: &PlayerSave) {
        if let Some(messages) = self.text.messages.as_mut() {
            process_messages(player_save, messages);
        }
    }

    pub fn update(&mut self, delta: f32) {
        if self.alive {
            self.text.update(delta);
        }
    }

    pub fn render(&self) {
        if self.alive {
            draw(self.background, self.pos.x, self.pos.y);
            self.text.render();
        }
    }

    pub fn input(&mut self) {
        self.text.input();
    }

}

impl Default for TextWindow {
    fn default() -> Self {
        let pos = Vec2::new(6.0, 116.0);
        Self {
            alive: false,
            pos,
            background: byte_texture(include_bytes!("../../assets/gui/message.png")),
            text: DynamicText::new(Vec2::new(11.0, 5.0), pos),
        }
    }
}

impl Entity for TextWindow {
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

impl Reset for TextWindow {
    fn reset(&mut self) {
        self.text.reset();
    }
}

impl Completable for TextWindow {
    fn is_finished(&self) -> bool {
        self.text.is_finished()
    }
}