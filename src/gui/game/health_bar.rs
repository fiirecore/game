use firecore_util::{Entity, Reset};
use macroquad::prelude::Vec2;
use crate::util::graphics::draw_rect;

pub struct HealthBar {
	
	alive: bool,

	pub pos: Vec2,
	pub panel: Vec2,

	width: f32,
	gap: f32,
	
}

impl HealthBar {

	pub const WIDTH: f32 = 48.0;

	pub const UPPER_COLOR: [f32; 4] = [88.0 / 255.0, 208.0 / 255.0, 128.0 / 255.0, 1.0];
	pub const LOWER_COLOR: [f32; 4] = [112.0 / 255.0, 248.0 / 255.0, 168.0 / 255.0, 1.0];
	
	pub fn new(pos: Vec2, panel: Vec2) -> HealthBar {
		HealthBar {
			
			alive: true,

			panel,
			pos,

			width: Self::WIDTH,
			gap: 0.0,

		}
	}

	pub fn get_hp_width(current: u16, max: u16) -> f32 {
		current as f32 * Self::WIDTH / max as f32
	}
	
	pub fn update_bar(&mut self, new_pokemon: bool, current: u16, max: u16) {
		
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

	pub fn render(&self, y_offset: f32) {
		draw_rect(Self::UPPER_COLOR, self.pos.x + self.panel.x, self.pos.y + self.panel.y + y_offset, self.width + self.gap, 1.0);
		draw_rect(Self::LOWER_COLOR, self.pos.x + self.panel.x, self.pos.y + self.panel.y + 1.0 + y_offset, self.width + self.gap, 2.0);
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