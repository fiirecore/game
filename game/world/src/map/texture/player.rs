use firecore_game::deps::hash::HashMap;
use firecore_game::util::{WIDTH, HEIGHT, TILE_SIZE, Direction};
use firecore_world_lib::character::Character;
use firecore_game::macroquad::prelude::{Texture2D, draw_texture_ex, WHITE, DrawTextureParams, Rect};
use firecore_game::graphics::byte_texture;
use firecore_world_lib::character::MoveType;

const SCREEN_X: f32 = (WIDTH as isize >> 1) as f32;
const SCREEN_Y: f32 = ((HEIGHT as isize - TILE_SIZE as isize) >> 1) as f32 - TILE_SIZE;

pub struct PlayerTexture {

	pub draw: bool,

	pub textures: HashMap<MoveType, CharacterTexture>,

	accumulator: f32,

	// pub walking_texture: Option<Texture2D>,
	// pub running_texture: Option<Texture2D>,
	// pub surfing_texture: Option<Texture2D>,
	
}

pub struct CharacterTexture {

	pub idle: Option<f32>,
	pub texture: Texture2D,

}

impl From<Texture2D> for CharacterTexture {
    fn from(texture: Texture2D) -> Self {
        Self {
			idle: None,
			texture,
		}
    }
}

impl PlayerTexture {

	pub fn load(&mut self) {
		self.textures.insert(MoveType::Walking, byte_texture(include_bytes!("../../../assets/player/walking.png")).into());
		self.textures.insert(MoveType::Running, byte_texture(include_bytes!("../../../assets/player/running.png")).into());
		self.textures.insert(MoveType::Swimming, CharacterTexture {
			idle: Some(0.5),
			texture: byte_texture(include_bytes!("../../../assets/player/surfing.png")),
		});
	}

	pub fn update(&mut self, delta: f32, character: &mut Character) {
		if !character.moving {
			if let Some(texture) = self.textures.get(&character.move_type) {
				if let Some(idle) = texture.idle {
					self.accumulator += delta;
					if self.accumulator > idle {
						self.accumulator -= idle;
						character.update_sprite();
					}
				}
			}
		}
	}

	pub fn render(&self, character: &Character) {
		if self.draw {
			if let Some(texture) = self.textures.get(&character.move_type) {
				let (x, width) = current_texture(character);
				draw_texture_ex(
					texture.texture, SCREEN_X - width / 2.0, SCREEN_Y, WHITE, DrawTextureParams {
						source: Some(Rect::new(
							x,
							0.0,
							width,
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

fn current_texture(character: &Character) -> (f32, f32) { // x, width
		let (indexes, width) = player_texture_index(character);
		((*indexes
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
			) as f32)
		* width, width)
}

pub const fn player_texture_index(character: &Character) -> ([u8; 4], f32) {
	match character.move_type {
	    MoveType::Walking => (match character.position.direction {
			Direction::Up => [1, 5, 1, 6],
			Direction::Down => [0, 3, 0, 4],
			_ => [2, 7, 2, 8],
		}, 16.0),
	    MoveType::Running => (match character.position.direction {
			Direction::Up => [6, 7, 6, 8],
			Direction::Down => [3, 4, 3, 5],
			_ => [9, 10, 9, 11],
		}, 16.0),
		MoveType::Swimming => (match character.position.direction {
			Direction::Up => [2, 2, 3, 3],
			Direction::Down => [0, 0, 1, 1],
			_ => [4, 4, 5, 5],
		}, 32.0),
	}
}

impl Default for PlayerTexture {
    fn default() -> Self {
        Self {
			draw: true,
			textures: HashMap::with_capacity(3),
			accumulator: 0.0,
		}
    }
}