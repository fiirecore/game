use firecore_pokedex::pokemon::Health;
use firecore_util::{Entity, Reset};
use macroquad::color_u8;
use macroquad::prelude::Color;
use macroquad::prelude::{Vec2, draw_rectangle};

pub struct HealthBar {
	
	alive: bool,

	origin: Vec2,
	pub offset: Vec2,

	width: f32,
	gap: f32,
	
}

impl HealthBar {

	pub const WIDTH: f32 = 48.0;

	pub const UPPER_COLOR: Color = color_u8!(88, 208, 128, 255);
	pub const LOWER_COLOR: Color = color_u8!(112, 248, 168, 255);
	
	pub fn new(origin: Vec2, offset: Vec2) -> HealthBar {
		HealthBar {
			
			alive: false,

			origin,
			offset,

			width: Self::WIDTH,
			gap: 0.0,

		}
	}

	pub fn get_hp_width(current: Health, max: Health) -> f32 {
		current as f32 * Self::WIDTH / max as f32
	}
	
	pub fn update_bar(&mut self, new_pokemon: bool, current: Health, max: Health) {
		
		let new = Self::get_hp_width(current, max);
		self.gap = if new_pokemon {
			0.0
		} else {
			self.width - new
		};
		self.width = new;
	}

	pub fn is_moving(&self) -> bool {
		return self.gap != 0.0;
	}
	
	pub fn update(&mut self, delta: f32) {
		if self.is_moving() {
			if self.gap > 0.0 {
				self.gap -= 60.0 * delta;
				if self.gap < 0.0 {
					self.gap = 0.0;
				}
			} else {
				self.gap += 60.0 * delta;
				if self.gap > 0.0 {
					self.gap = 0.0;
				}
			}
		}
	}

	pub fn render(&self) {
		let x = self.origin.x + self.offset.x;
		let y = self.origin.y + self.offset.y;// + y_offset;
		let w = self.width + self.gap;
		draw_rectangle(x, y, w, 1.0, Self::UPPER_COLOR);
		draw_rectangle(x, y + 1.0, w, 2.0, Self::LOWER_COLOR);
	}

}

impl Entity for HealthBar {

	fn spawn(&mut self) {
		self.alive = true;
		// self.reset();
	}
	
	fn despawn(&mut self) {
		self.alive = true;
		self.reset();
	}
	
	fn is_alive(& self) -> bool {
		self.alive
	}
	
}

impl Reset for HealthBar {
	fn reset(&mut self) {
		self.width = Self::WIDTH;
		self.gap = 0.0;
	}
}