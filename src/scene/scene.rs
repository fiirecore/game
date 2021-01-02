use opengl_graphics::GlGraphics;
use piston_window::Context;
pub use crate::engine::text::TextRenderer;
use crate::engine::game_context::GameContext;

pub trait Scene {
	
	fn load(&mut self) {
		
	}

	fn on_start(&mut self, _context: &mut GameContext) {

	}
	
	fn update(&mut self, _context: &mut GameContext) {
		
	}
	
	fn render(&mut self, _ctx: &mut Context, _g: &mut GlGraphics, _tr: &mut TextRenderer) {
		
	}
	
	fn input(&mut self, _context: &mut GameContext) {
		
	}
	
	fn dispose(&mut self) {
		
	}
	
	fn name(&self) -> &str;
	
	fn next_scene(&self) -> usize;
	
}

pub struct DefaultScene;

impl DefaultScene {
	pub fn new() -> DefaultScene {
		DefaultScene {}
	}
}

impl Scene for DefaultScene {
	
	fn name(&self) -> &str {
		&"Default"
	}
	
	fn next_scene(&self) -> usize {
		1
	}
	
}