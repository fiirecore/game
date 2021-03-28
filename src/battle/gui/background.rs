use macroquad::prelude::Texture2D;
use crate::util::graphics::{byte_texture, draw};

pub struct BattleBackground {

	background_texture: Texture2D,
	ground_texture: Texture2D,

}

impl BattleBackground {

    pub fn new() -> Self {
        Self {
            background_texture: byte_texture(include_bytes!("../../../build/assets/battle/background.png")),
            ground_texture: byte_texture(include_bytes!("../../../build/assets/battle/ground.png")),
        }

    }

    pub fn render(&self, offset: f32) {
        draw(self.background_texture, 0.0, 1.0);
        draw(self.ground_texture, 113.0 - offset, 50.0);
		draw(self.ground_texture, offset, 103.0);
    }

}