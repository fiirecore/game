use firecore_world::character::player::PlayerCharacter;
use macroquad::prelude::DrawTextureParams;

use firecore_util::Direction;
use crate::util::graphics::Texture;
use crate::util::TILE_SIZE;

static SCREEN_X: f32 = ((crate::BASE_WIDTH as isize - TILE_SIZE as isize) >> 1) as f32 + 0.0;
static SCREEN_Y: f32 = ((crate::BASE_HEIGHT as isize - TILE_SIZE as isize) >> 1) as f32 - 16.0;

pub struct PlayerTexture {

	pub draw: bool,

	pub walking_texture: Option<Texture>,
	pub running_texture: Option<Texture>,
	
}

impl PlayerTexture {

	pub fn render(&self, character: &PlayerCharacter) {
		if self.draw {
			if let Some(texture) = if character.properties.running {self.running_texture} else {self.walking_texture} {
				macroquad::prelude::draw_texture_ex(
					texture, SCREEN_X, SCREEN_Y, macroquad::prelude::WHITE, DrawTextureParams {
						source: Some(macroquad::prelude::Rect::new(
							self.current_texture_pos(character),
							0.0,
							16.0,
							32.0,
						)),
						flip_x: character.position.local.direction == Direction::Right,
						..Default::default()
					}
				)
			}
		}
	}

	fn current_texture_pos(&self, character: &PlayerCharacter) -> f32 {
		(
			*self.texture_index(character)
				.get(
					(
						if character.position.local.offset.x != 0.0 {
							character.position.local.offset.x
						} else {
							character.position.local.offset.y
						}.abs() as usize >> 3
					) + character.properties.sprite_index as usize
				).unwrap_or(
					&3
				)
			<< 4
		) as f32
	}

	pub const fn texture_index(&self, character: &PlayerCharacter) -> [u8; 4] {
		if character.properties.running {
			match character.position.local.direction {
			    Direction::Up => [6, 7, 6, 8],
			    Direction::Down => [3, 4, 3, 5],
			    _ => [9, 10, 9, 11],
			}
		} else {
			match character.position.local.direction {
			    Direction::Up => [1, 5, 1, 6],
			    Direction::Down => [0, 3, 0, 4],
			    _ => [2, 7, 2, 8],
			}
		}
	}
	
}

impl Default for PlayerTexture {
    fn default() -> Self {
        Self {
			draw: true,
			walking_texture: None,
			running_texture: None,
		}
    }
}