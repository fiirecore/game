use firecore_util::text::TextColor;
use macroquad::prelude::Vec2;

use crate::util::graphics;
pub struct StaticText {
	
	alive: bool,
	pos: Vec2,
	pub panel: Vec2,

	pub text: Vec<String>,
	pub color: TextColor,
	pub font_id: usize,

	direction: bool,
	
}

impl StaticText {
	
	pub fn new(text: Vec<String>, text_color: TextColor, font_id: usize, from_right: bool, pos: Vec2, panel: Vec2) -> Self {
		
		Self {
			
			alive: false,
			pos,
			panel,

			text,
			color: text_color,
			font_id,

			direction: from_right,
			
		}
		
	}
	
	pub fn render(&self) {
		for (index, string) in self.text.iter().enumerate() {
			if self.direction {
				graphics::draw_text_right(self.font_id, string, self.color, self.panel.x + self.pos.x, self.panel.y + self.pos.y + (index << 4) as f32);
			} else {
				graphics::draw_text_left(self.font_id, string, self.color, self.panel.x + self.pos.x, self.panel.y + self.pos.y + (index << 4) as f32);
			}
		}		
	}

}

impl firecore_util::Entity for StaticText {

	fn spawn(&mut self) {
		self.alive = true;		
	}
	
	fn despawn(&mut self) {
		self.alive = false;
	}
	
	fn is_alive(& self) -> bool {
		self.alive
	}

}