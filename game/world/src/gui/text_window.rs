use firecore_game::util::{Entity, Reset, Completable};
use firecore_game::macroquad::prelude::{Vec2, Texture2D};

use firecore_game::gui::text::DynamicText;
use firecore_game::text::Message;
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

    pub fn set_text(&mut self, message: Message) {
        self.text.message = message;
    }

    pub fn process_messages(&mut self, save: &firecore_game::storage::player::PlayerSave) {
        self.text.process_messages(save);
    }

    pub fn update(&mut self, delta: f32) {
        if self.alive {
            self.text.update(delta, #[cfg(debug_assertions)] "update");
        }
    }

    pub fn render(&self) {
        if self.alive {
            draw(self.background, self.pos.x, self.pos.y);
            self.text.render(#[cfg(debug_assertions)] "render");
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
            #[deprecated]
            text: DynamicText::new(Vec2::new(11.0, 5.0), pos, 1, firecore_game::text::TextColor::Black, 5, "wrldwndw"),
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