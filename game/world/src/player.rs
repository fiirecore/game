use firecore_game::util::{WIDTH, HEIGHT, TILE_SIZE, Direction};
use firecore_world_lib::character::Character;
use firecore_game::macroquad::prelude::{Texture2D, draw_texture_ex, WHITE, DrawTextureParams, Rect};
use firecore_game::graphics::byte_texture;

const SCREEN_X: f32 = ((WIDTH as isize - TILE_SIZE as isize) >> 1) as f32;
const SCREEN_Y: f32 = ((HEIGHT as isize - TILE_SIZE as isize) >> 1) as f32 - TILE_SIZE;

pub struct PlayerTexture {

	pub draw: bool,

	pub walking_texture: Option<Texture2D>,
	pub running_texture: Option<Texture2D>,
	
}

impl PlayerTexture {

	pub fn load(&mut self) {
		self.walking_texture = Some(byte_texture(include_bytes!("../assets/player/walking.png")));
		self.running_texture = Some(byte_texture(include_bytes!("../assets/player/running.png")));
	}

	pub fn render(&self, character: &Character) {
		if self.draw {
			if let Some(texture) = if character.running {self.running_texture} else {self.walking_texture} {
				draw_texture_ex(
					texture, SCREEN_X, SCREEN_Y, WHITE, DrawTextureParams {
						source: Some(Rect::new(
							current_texture_pos(character),
							0.0,
							16.0,
							32.0,
						)),
						flip_x: character.position.direction == Direction::Right,
						..Default::default()
					}
				)
			}
		}
	}
	
}

fn current_texture_pos(character: &Character) -> f32 {
	(
		*texture_index(character)
			.get(
				(
					if if character.position.offset.x != 0.0 {
						character.position.offset.x
					} else {
						character.position.offset.y
					}.abs() < 8.0 && character.moving { 1 } else { 0 }//.abs() as usize >> 3
				) + character.sprite_index as usize
			).unwrap_or(
				&3
			)
		<< 4
	) as f32
}

pub const fn texture_index(character: &Character) -> [u8; 4] {
	if character.running {
		match character.position.direction {
			Direction::Up => [6, 7, 6, 8],
			Direction::Down => [3, 4, 3, 5],
			_ => [9, 10, 9, 11],
		}
	} else {
		match character.position.direction {
			Direction::Up => [1, 5, 1, 6],
			Direction::Down => [0, 3, 0, 4],
			_ => [2, 7, 2, 8],
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