use opengl_graphics::GlGraphics;
use piston_window::Context;
pub use crate::engine::text::TextRenderer;
use crate::engine::game_context::GameContext;
//use async_trait::async_trait;

pub trait Scene: SceneLoad {
	
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

//#[async_trait]
pub trait SceneLoad {

	fn load(&mut self, context: &mut GameContext);

	fn on_start(&mut self, context: &mut GameContext);

}