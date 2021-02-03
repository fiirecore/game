use crate::util::graphics::Texture;
use crate::util::graphics::draw;
use crate::util::graphics::texture::byte_texture;
use crate::util::Load;

pub struct PlayerBattleIntro {

	player_textures: Vec<Texture>,
	player_x_counter: f32,
	player_texture_index: u8,

}

impl PlayerBattleIntro {

    pub fn new() -> Self {

        Self {
			player_textures: vec! {
                byte_texture(include_bytes!("../../../../../build/assets/gui/battle/player0.png")),
                byte_texture(include_bytes!("../../../../../build/assets/gui/battle/player1.png")),
                byte_texture(include_bytes!("../../../../../build/assets/gui/battle/player2.png")),
                byte_texture(include_bytes!("../../../../../build/assets/gui/battle/player3.png")),
                byte_texture(include_bytes!("../../../../../build/assets/gui/battle/player4.png")),
            },
			player_x_counter: 0.0,
			player_texture_index: 0,
        }

    }

    pub fn should_update(&self) -> bool {
        return self.player_x_counter < 41.0 + 63.0;
    }

    pub fn update(&mut self, delta: f32) {
        if self.player_texture_index == 0 {
            self.player_texture_index = 1;
        } else if self.player_texture_index == 1 && self.player_x_counter >= 42.0 {
            self.player_texture_index = 2;
        } else if self.player_texture_index == 2 && self.player_x_counter >= 60.0 {
            self.player_texture_index = 3;
        } else if self.player_texture_index == 3 && self.player_x_counter >= 78.0 {
            self.player_texture_index = 4;
        } else {
            self.player_x_counter += 180.0 * delta;
        }
    }

    pub fn draw(&self, offset: f32) {
        draw(self.player_textures[self.player_texture_index as usize], 41.0 + offset - self.player_x_counter, 64.0);
    }

    pub fn reset(&mut self) {
        self.player_x_counter = 0.0;
		self.player_texture_index = 0;
    }

}

impl Load for PlayerBattleIntro {

    fn load(&mut self) {

    }

    fn on_start(&mut self) {
        self.reset();
    }

}