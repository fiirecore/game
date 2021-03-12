use macroquad::prelude::DrawTextureParams;

use firecore_util::{GlobalPosition, Direction};
use crate::util::graphics::Texture;
use crate::io::data::player::PlayerData;
use crate::util::TILE_SIZE;

static SCREEN_X: f32 = ((crate::BASE_WIDTH as isize - TILE_SIZE as isize) >> 1) as f32 + 0.0;
static SCREEN_Y: f32 = ((crate::BASE_HEIGHT as isize - TILE_SIZE as isize) >> 1) as f32 - 16.0;

// static TEX_TICK_LENGTH: f32 = 8.0 / 60.0;
pub static BASE_SPEED: u8 = 1;
pub static RUN_SPEED: u8 = BASE_SPEED << 1;

#[derive(Default)]
pub struct Player {
	
	pub position: GlobalPosition,

	pub speed: u8,

	pub walking_texture: Option<Texture>,
	pub running_texture: Option<Texture>,
	sprite_index: u8,
	
	pub moving: bool,
	pub running: bool,
	pub frozen: bool,

	pub noclip: bool,
	
}

impl Player {

	pub fn new(data: &PlayerData) -> Player {
		
		Player {
			
			position: data.location.position,
			speed: BASE_SPEED,

			..Default::default()
			
		}
		
	}

	pub fn render(&self) {
		if let Some(texture) = if self.running {self.running_texture} else {self.walking_texture} {
			macroquad::prelude::draw_texture_ex(
				texture, SCREEN_X, SCREEN_Y, macroquad::prelude::WHITE, DrawTextureParams {
				    source: Some(macroquad::prelude::Rect::new(
						self.current_texture_pos(),
						0.0,
						16.0,
						32.0,
					)),
				    flip_x: self.position.local.direction == Direction::Right,
					..Default::default()
				}
			)
		}
		
	}

	pub fn on_try_move(&mut self, direction: Direction) {
		self.position.local.direction = direction;
		if self.sprite_index == 0 {
			self.sprite_index = 2;
		} else {
			self.sprite_index = 0;
		}
	}

	pub fn freeze(&mut self) {
		self.frozen = true;
		self.position.local.offset.x = 0.0;
		self.position.local.offset.y = 0.0;
		self.moving = false;
		self.running = false;
		self.speed = BASE_SPEED;
	}

	fn current_texture_pos(&self) -> f32 {
		(
			*self.texture_index()
				.get(
					(
						if self.position.local.offset.x != 0.0 {
							self.position.local.offset.x
						} else {
							self.position.local.offset.y
						}.abs() as usize >> 3
					) + self.sprite_index as usize
				).unwrap_or(
					&3
				)
			<< 4
		) as f32
	}

	pub const fn texture_index(&self) -> [u8; 4] {
		if self.running {
			match self.position.local.direction {
			    Direction::Up => [6, 7, 6, 8],
			    Direction::Down => [3, 4, 3, 5],
			    _ => [9, 10, 9, 11],
			}
		} else {
			match self.position.local.direction {
			    Direction::Up => [1, 5, 1, 6],
			    Direction::Down => [0, 3, 0, 4],
			    _ => [2, 7, 2, 8],
			}
		}
	}
	
}