use crate::engine::game_context::GameContext;
use crate::io::data::Direction;
use std::path::PathBuf;
use opengl_graphics::GlGraphics;
use piston_window::Context;

use crate::engine::engine::Texture;
use crate::engine::text::TextRenderer;

use crate::util::file_util::asset_as_pathbuf;
use crate::util::texture_util::texture_from_path;
use crate::util::render_util::draw_o;

pub struct Panel {
	
	alive: bool,
	pub components: Vec<Box<dyn GuiComponent>>,
	
	pub x: isize,
	pub y: isize,
	
	background_texture: Option<Texture>,
	texture_path: PathBuf,
	
}

impl Panel {
	
	pub fn new(texture_path: &str, x: isize, y: isize) -> Panel {
		
		Panel {
			
			alive: false,
			components: Vec::new(),
			
			x: x,
			y: y,
			
			background_texture: None,
			texture_path: asset_as_pathbuf(texture_path),
			
		}
		
	}
	
	/*
	
	pub fn add(&mut self, component: Box<dyn GuiComponent>) {
		self.components.push(component);
	}
	
	*/
	
}

impl GuiComponent for Panel {
	
	fn load(&mut self) {
		self.background_texture = Some(texture_from_path(&self.texture_path));
	}
	
	fn enable(&mut self) {
		self.alive = true;
		for component in self.components.as_mut_slice() {
			component.enable();
		}
		
	}
	
	fn disable(&mut self) {
		self.alive = false;
		for component in self.components.as_mut_slice() {
			component.disable();
		}
	}
	
	fn is_active(& self) -> bool {
		self.alive
	}

	fn update(&mut self, context: &mut GameContext) {
		for component in self.components.as_mut_slice() {
			component.update(context);
		}
	}
	
	fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
		draw_o(ctx, g, self.background_texture.as_ref(), self.x as isize, self.y as isize);
		for component in &self.components {
			component.render(ctx, g, tr);
		}
	}

	fn update_position(&mut self, x: isize, y: isize) {
		self.x = x;
		self.y = y;
		for component in &mut self.components {
			component.update_position(x, y);
		}
	}
}

pub trait GuiComponent {

	fn load(&mut self) {

	}

	fn enable(&mut self);

	fn disable(&mut self);

	fn is_active(&self) -> bool;

	fn update(&mut self, _context: &mut GameContext) {

	}

	fn render(&self, _ctx: &mut Context, _g: &mut GlGraphics, _tr: &mut TextRenderer) {

	}

	fn update_position(&mut self, x: isize, y: isize);
	
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

	fn input(&mut self, context: &mut GameContext);

	fn next(&self) -> u8;

}

#[derive(Clone)]
pub struct BasicText {
	
	alive: bool,
	x: isize,
	y: isize,
	panel_x: isize,
	panel_y: isize,

	pub text: Vec<String>,
	pub font_id: usize,

	direction: Direction,
	
}

impl BasicText {
	
	pub fn new(text: Vec<String>, font_id: usize, direction: Direction, x: isize, y: isize, panel_x: isize, panel_y: isize) -> BasicText {
		
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
	
	fn update_position(&mut self, x: isize, y: isize) {
		self.panel_x = x;
		self.panel_y = y;
	}
	
	fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
		for line_index in 0..self.get_text().len() {
			if self.direction == Direction::Right {
				tr.render_text_from_right(ctx, g, self.get_font_id(), self.get_line(line_index), self.panel_x + self.x, self.panel_y + self.y + line_index as isize * 16);
			} else {
				tr.render_text_from_left(ctx, g, self.get_font_id(), self.get_line(line_index), self.panel_x + self.x, self.panel_y + self.y + line_index as isize * 16);
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