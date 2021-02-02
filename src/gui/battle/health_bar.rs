



use crate::gui::GuiComponent;

use crate::util::render::draw_rect;

static UPPER_COLOR: [f32; 4] = [88.0 / 255.0, 208.0 / 255.0, 128.0 / 255.0, 1.0];
static LOWER_COLOR: [f32; 4] = [112.0 / 255.0, 248.0 / 255.0, 168.0 / 255.0, 1.0];

static WIDTH: u8 = 48;

pub struct HealthBar {
	
	alive: bool,

	x: f32, 
	y: f32,
	panel_x: f32,
	panel_y: f32,

	width: u8,
	previous_width: u8,

	has_width: bool,
	
}

impl HealthBar {
	
	pub fn new(x: f32, y: f32, panel_x: f32, panel_y: f32) -> HealthBar {

		HealthBar {
			
			alive: true,

			x: x,
			y: y,

			panel_x: panel_x,
			panel_y: panel_y,

			width: WIDTH,
			previous_width: WIDTH,

			has_width: false,
			
		}
		
	}
	
	pub fn update_bar(&mut self, current_health: u16, max_health: u16) {
		self.previous_width = self.width;
		self.width = (current_health as f32 * 48f32 / max_health as f32).ceil() as u8;
		if !self.has_width {
			self.previous_width = self.width;
			self.has_width = true;
		}
	}

	pub fn is_moving(&self) -> bool {
		return self.previous_width > self.width;
	}

	pub fn get_width(&self) -> u8 {
		return self.previous_width;
	}

	pub fn reset(&mut self) {
		self.width = WIDTH;
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

	fn update(&mut self, _delta: f32) {
		if self.is_moving() {
			self.previous_width -= 1;
		}
	}

	fn render(&self) {
		draw_rect(UPPER_COLOR, self.x + self.panel_x, self.y + self.panel_y, self.get_width() as u32, 1);
		draw_rect(LOWER_COLOR, self.x + self.panel_x, self.y + self.panel_y + 1.0, self.get_width() as u32, 2);
	}
	
}