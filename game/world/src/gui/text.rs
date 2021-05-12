use game::{
    util::{Entity, Reset, Completable},
    text::Message,
    gui::DynamicText,
    graphics::{byte_texture, draw},
    macroquad::prelude::{Vec2, const_vec2, Texture2D}
};

pub struct TextWindow {

    alive: bool,
    background: Texture2D,
    text: DynamicText,

}

impl TextWindow {

    const ORIGIN: Vec2 = const_vec2!([6.0, 116.0]);
    const TEXT_OFFSET: Vec2 = const_vec2!([11.0, 5.0]);

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
            self.text.update(delta);
        }
    }

    pub fn render(&self) {
        if self.alive {
            draw(self.background, Self::ORIGIN.x, Self::ORIGIN.y);
            self.text.render();
        }
    }

}

impl Default for TextWindow {
    fn default() -> Self {
        Self {
            alive: false,
            // origin: Self::ORIGIN,
            background: byte_texture(include_bytes!("../../assets/gui/message.png")),
            // #[deprecated]
            text: DynamicText::new(Self::ORIGIN + Self::TEXT_OFFSET, 1, firecore_game::text::TextColor::Black, 5),
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