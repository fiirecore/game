use game::{
    macroquad::prelude::Texture2D,
    graphics::{byte_texture, draw},
};

static mut PANEL: Option<Texture2D> = None;

pub fn panel_texture() -> Texture2D {
	unsafe { *PANEL.get_or_insert(byte_texture(include_bytes!("../../assets/gui/panel.png"))) }
}

pub struct BattleBackground {

	background: Texture2D,
	ground: Texture2D,
	pub panel: Texture2D,

}

impl BattleBackground {

    pub fn new() -> Self {
        Self {
            background: byte_texture(include_bytes!("../../assets/background.png")),
            ground: byte_texture(include_bytes!("../../assets/ground.png")),
            panel: panel_texture(),
        }

    }

    pub fn render(&self, offset: f32) {
        draw(self.background, 0.0, 1.0);
        draw(self.ground, 113.0 - offset, 50.0);
		draw(self.ground, offset, 103.0);
    }

}