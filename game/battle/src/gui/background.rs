use game::{
    macroquad::prelude::Texture2D,
    graphics::{byte_texture, draw},
};

pub struct BattleBackground {

	background_texture: Texture2D,
	ground_texture: Texture2D,

}

impl BattleBackground {

    pub fn new() -> Self {
        Self {
            background_texture: byte_texture(include_bytes!("../../assets/background.png")),
            ground_texture: byte_texture(include_bytes!("../../assets/ground.png")),
        }

    }

    pub fn render(&self, offset: f32) {
        draw(self.background_texture, 0.0, 1.0);
        draw(self.ground_texture, 113.0 - offset, 50.0);
		draw(self.ground_texture, offset, 103.0);
    }

}