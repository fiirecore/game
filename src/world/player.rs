use firecore_world::npc::movement::NPCDestination;
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

	pub destination: Option<NPCDestination>,
	
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

	pub fn do_move(&mut self, delta: f32) -> bool {
		if self.position.local.offset.x != 0.0 || self.position.local.offset.y != 0.0 {
            match self.position.local.direction {
                Direction::Up => {
                    self.position.local.offset.y -= (self.speed as f32) * 60.0 * delta;
                    if self.position.local.offset.y <= -16.0 {
                        self.position.local.coords.y -= 1;
                        self.position.local.offset.y = 0.0;
                        return true;
                    }
                }
                Direction::Down => {
                    self.position.local.offset.y += (self.speed as f32) * 60.0 * delta;
                    if self.position.local.offset.y >= 16.0 {
                        self.position.local.coords.y += 1;
                        self.position.local.offset.y = 0.0;
                        return true;
                    }
                }
                Direction::Left => {
                    self.position.local.offset.x -= (self.speed as f32) * 60.0 * delta;
                    if self.position.local.offset.x <= -16.0 {
                        self.position.local.coords.x -= 1;
                        self.position.local.offset.x = 0.0;
                        return true;
                    }
                }
                Direction::Right => {
                    self.position.local.offset.x += (self.speed as f32) * 60.0 * delta;
                    if self.position.local.offset.x >= 16.0 {
                        self.position.local.coords.x += 1;
                        self.position.local.offset.x = 0.0;
                        return true;
                    }
                }
            }
        }
		false
	}

	pub fn should_move_to(&mut self) -> bool {
		if let Some(offset) = self.destination.as_ref() {
            self.position.local.coords != offset.coords
        } else {
            false
        }
	}

	pub fn move_to_destination(&mut self, delta: f32) {
		if let Some(offset) = self.destination.as_mut() {

            if self.position.local.coords.y == offset.coords.y {
                self.position.local.direction = if self.position.local.coords.x < offset.coords.x {
                    Direction::Right
                } else {
                    Direction::Left
                };
            } else if self.position.local.coords.x == offset.coords.x {
                self.position.local.direction = if self.position.local.coords.y < offset.coords.y {
                    Direction::Down
                } else {
                    Direction::Up
                };
            }

            let offsets = self.position.local.direction.offset_f32();
            let offset = 60.0 * self.speed as f32 * delta;
            self.position.local.offset.x += offsets.x * offset;
            self.position.local.offset.y += offsets.y * offset;

            if self.position.local.offset.y * offsets.y >= 16.0 {
                self.position.local.coords.y += offsets.y as isize;
                self.position.local.offset.y = 0.0;
				self.change_sprite_index();
            }
            
            if self.position.local.offset.x * offsets.x >= 16.0 {
                self.position.local.coords.x += offsets.x as isize;
                self.position.local.offset.x = 0.0;
				self.change_sprite_index();
            }
            
        }
	}

	pub fn on_try_move(&mut self, direction: Direction) {
		self.position.local.direction = direction;
		self.change_sprite_index();
	}

	fn change_sprite_index(&mut self) {
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