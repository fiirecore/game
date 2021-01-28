use crate::io::data::Direction;
use crate::util::render::draw;
use crate::util::texture::Texture;
use crate::util::text_renderer::TextRenderer;

pub struct Background {
	
	alive: bool,
	
	pub x: f32,
	pub y: f32,
	
	texture: Texture,
	
}

impl Background {
	
	pub fn new(texture: Texture, x: f32, y: f32) -> Self {
		
		Self {
			
			alive: false,
			
			x: x,
			y: y,
			
			texture: texture,
			
		}
		
	}
	
}

impl GuiComponent for Background {
	
	fn load(&mut self) {
	}
	
	fn enable(&mut self) {
		self.alive = true;
	}
	
	fn disable(&mut self) {
		self.alive = false;
	}
	
	fn is_active(& self) -> bool {
		self.alive
	}
	
	fn render(&self, _tr: &TextRenderer) {
		draw(self.texture, self.x, self.y);
	}

	fn update_position(&mut self, x: f32, y: f32) {
		self.x = x;
		self.y = y;
	}
}

pub trait GuiComponent {

	fn load(&mut self) {}

	fn enable(&mut self);

	fn disable(&mut self);

	fn is_active(&self) -> bool;

	fn update(&mut self, _delta: f32) {}

	fn render(&self, tr: &TextRenderer);

	fn update_position(&mut self, x: f32, y: f32);
	
}

pub trait GuiText: GuiComponent {
	
	fn get_line(&self, index: usize) -> &String;

	fn get_text(&self) -> &Vec<String>;
	
	fn get_font_id(&self) -> usize;

}

pub trait GuiButton: GuiComponent {
	
	fn on_use(&mut self);
	
}

pub trait Activatable {

	fn focus(&mut self);

	fn unfocus(&mut self);

	fn in_focus(&mut self) -> bool;

	fn input(&mut self, delta: f32);

	fn next(&self) -> u8;

}

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

impl GuiComponent for BasicText {

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

/*
pub struct BasicButton {
	
	text: String,
	
}

impl BasicButton {
	
	pub fn new(text: &str) -> BasicButton {
		
		BasicButton {
			
			text: String::from(text),
			
		}
		
	}
	
}
*/