use opengl_graphics::GlGraphics;
use piston_window::Context;
use crate::util::context::GameContext;
use crate::util::text_renderer::TextRenderer;

pub mod util;

pub mod entities {
	pub mod player;
}

pub mod texture {
	pub mod still_texture_manager;
	pub mod movement_texture;
	pub mod movement_texture_manager;
	pub mod texture_manager;
	pub mod four_way_texture;
	pub mod three_way_texture;
}

pub trait Entity {
	
	fn spawn(&mut self);
	
	fn despawn(&mut self);
	
	fn is_alive(&self) -> bool;
	
}

pub trait Ticking {

	fn update(&mut self, _context: &mut GameContext) {
		
	}
	
	fn render(&self, _ctx: &mut Context, _g: &mut GlGraphics, _tr: &mut TextRenderer) {
		
	}

}