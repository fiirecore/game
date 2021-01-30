
use crate::util::texture::Texture;
use crate::util::render::draw;
use crate::util::texture::byte_texture;

pub struct BattleBackground {

	background_texture: Texture,
	ground_texture: Texture,

}

impl BattleBackground {

    pub fn new() -> Self {
        Self {
            background_texture: byte_texture(include_bytes!("../../../build/assets/gui/battle/background.png")),
            ground_texture: byte_texture(include_bytes!("../../../build/assets/gui/battle/grass_pad.png")),
        }

    }

    pub fn render(&self, offset: f32) {
        draw(self.background_texture, 0.0, 1.0);
        draw(self.ground_texture, 113.0 - offset, 50.0);
		draw(self.ground_texture, offset, 103.0);
    }

}