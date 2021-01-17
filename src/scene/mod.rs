use opengl_graphics::GlGraphics;
use piston_window::Context;
use crate::util::text_renderer::TextRenderer;
use crate::util::context::GameContext;
//use async_trait::async_trait;


pub mod scene_manager;
pub mod scenes {
	pub mod first_scene;
	pub mod character_creation_scene;
	pub mod firsttime_scenes;
	pub mod game_scene;
	pub mod loading_scenes;
	pub mod main_menu_scene;
	pub mod title_scene;
}

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