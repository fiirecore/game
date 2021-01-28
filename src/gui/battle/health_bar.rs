


use crate::util::text_renderer::TextRenderer;
use crate::gui::gui::GuiComponent;

use crate::util::render::draw_rect;

static GREEN1: [f32; 4] = [88.0 / 255.0, 208.0 / 255.0, 128.0 / 255.0, 1.0];
static GREEN2: [f32; 4] = [112.0 / 255.0, 248.0 / 255.0, 168.0 / 255.0, 1.0];

pub struct HealthBar {
	
	alive: bool,

	x: f32, 
	y: f32,

	panel_x: f32,
	panel_y: f32,

	width: u8,

	upper_hp_color: [f32; 4],
	lower_hp_color: [f32; 4],

	previous_width: u8,
	
}

impl HealthBar {
	
	pub fn new(x: f32, y: f32, panel_x: f32, panel_y: f32) -> HealthBar {
		
		let width = 48;

		HealthBar {
			
			alive: true,

			x: x,
			y: y,

			panel_x: panel_x,
			panel_y: panel_y,

			width: width,

			upper_hp_color: GREEN1,
			lower_hp_color: GREEN2,

			previous_width: width,
			
		}
		
	}
	
	pub fn update_bar(&mut self, current_health: u16, max_health: u16) {
		self.previous_width = self.width;
		self.width = (current_health as f32 * 48f32 / max_health as f32).ceil() as u8;
	}

	pub fn update(&mut self) {
		if self.is_moving() {
			self.previous_width -= 1;
		}		
	}

	pub fn is_moving(&self) -> bool {
		return self.previous_width > self.width;
	}

	pub fn get_width(&self) -> u8 {
		return self.previous_width;
	}

	fn reset(&mut self) {
		self.width = 48;
		self.previous_width = self.width;
	}
	
}

impl GuiComponent for HealthBar {
	
	fn enable(&mut self) {
		self.alive = true;
		self.reset();
	}
	
	fn disable(&mut self) {
		self.alive = true;
		self.reset();
	}
	
	fn is_active(& self) -> bool {
		self.alive
	}

	fn update_position(&mut self, x: f32, y: f32) {
		self.panel_x = x;
		self.panel_y = y;
	}

	fn render(&self, _tr: &TextRenderer) {
		draw_rect(self.upper_hp_color, self.x + self.panel_x, self.y + self.panel_y, self.get_width() as u32, 1);
		draw_rect(self.lower_hp_color, self.x + self.panel_x, self.y + self.panel_y + 1.0, self.get_width() as u32, 2);
	}
	
}