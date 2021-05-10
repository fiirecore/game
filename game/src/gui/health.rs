use pokedex::pokemon::Health;
use firecore_util::Reset;
use macroquad::prelude::{Color, color_u8, Vec2, draw_rectangle};

pub struct HealthBar {

	origin: Vec2,

	width: f32,
	gap: f32,
	
}

impl HealthBar {

	pub const WIDTH: f32 = 48.0;

	pub const UPPER: Color = color_u8!(88, 208, 128, 255);
	pub const LOWER: Color = color_u8!(112, 248, 168, 255);
	
	pub const fn new(origin: Vec2) -> HealthBar {
		HealthBar {

			origin,

			width: Self::WIDTH,
			gap: 0.0,

		}
	}

	pub fn get_hp_width(current: Health, max: Health) -> f32 {
		current as f32 * Self::WIDTH / max as f32
	}
	
	pub fn update_bar(&mut self, current: Health, max: Health, reset: bool) -> bool {
		
		let new = Self::get_hp_width(current, max);

		let change = new != self.width;

		self.gap = if reset {
			0.0
		} else {
			self.width - new
		};
		self.width = new;

		change
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
		let w = self.width + self.gap;
		draw_rectangle(self.origin.x, self.origin.y, w, 1.0, Self::UPPER);
		draw_rectangle(self.origin.x, self.origin.y + 1.0, w, 2.0, Self::LOWER);
	}

	pub fn render_position(&self, pos: Vec2) {
		let w = self.width + self.gap;
		draw_rectangle(pos.x, pos.y, w, 1.0, Self::UPPER);
		draw_rectangle(pos.x, pos.y + 1.0, w, 2.0, Self::LOWER);
	}

}

impl Reset for HealthBar {
	fn reset(&mut self) {
		self.width = Self::WIDTH;
		self.gap = 0.0;
	}
}