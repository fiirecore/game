use crate::io::data::Direction;
use crate::util::text_renderer::TextRenderer;

use super::GuiText;

#[derive(Clone)]
pub struct BasicText {
	
	alive: bool,
	x: f32,
	y: f32,
	panel_x: f32,
	panel_y: f32,

	pub text: Vec<String>,
	pub font_id: usize,

	direction: Direction,
	
}

impl BasicText {
	
	pub fn new(text: Vec<String>, font_id: usize, direction: Direction, x: f32, y: f32, panel_x: f32, panel_y: f32) -> BasicText {
		
		BasicText {
			
			alive: false,
			x: x,
			y: y,
			panel_x: panel_x,
			panel_y: panel_y,

			text: text,
			font_id: font_id,

			direction: direction,
			
		}
		
	}
	
}

impl super::GuiComponent for BasicText {

	fn enable(&mut self) {
		self.alive = true;		
	}
	
	fn disable(&mut self) {
		self.alive = false;
	}
	
	fn is_active(& self) -> bool {
		self.alive
	}
	
	fn update_position(&mut self, x: f32, y: f32) {
		self.panel_x = x;
		self.panel_y = y;
	}
	
	fn render(&self, tr: &TextRenderer) {
		for line_index in 0..self.get_text().len() {
			if self.direction == Direction::Right {
				tr.render_text_from_right(self.get_font_id(), self.get_line(line_index), self.panel_x + self.x, self.panel_y + self.y + (line_index << 4) as f32);
			} else {
				tr.render_text_from_left(self.get_font_id(), self.get_line(line_index), self.panel_x + self.x, self.panel_y + self.y + (line_index << 4) as f32);
			}
		}
		
		
	}
	
}

impl GuiText for BasicText {
	
	fn get_line(&self, index: usize) -> &String {
		&self.get_text()[index]
	}

	fn get_text(&self) -> &Vec<String> {
		&self.text
	}

	fn get_font_id(&self) -> usize {
		self.font_id
	}
	
}