use opengl_graphics::GlGraphics;
use piston_window::Context;
use crate::engine::text::TextRenderer;
use crate::engine::game_context::GameContext;

use crate::scene::scene::Scene;

use crate::game::game_manager::GameManager;
use crate::scene::scene::SceneLoad;

pub struct GameScene {

	scene_token: usize,
	
	game_manager: GameManager,

}

impl GameScene {
	
	pub fn new() -> GameScene {

		GameScene {
			
			scene_token: 0,

			game_manager: GameManager::new(),

		}

	}
	
}


impl SceneLoad for GameScene {

	fn load(&mut self, _context: &mut GameContext) {
		self.game_manager.load();
	}

	fn on_start(&mut self, context: &mut GameContext) {
		self.game_manager.on_start(context);
	}
}

impl Scene for GameScene {
	
	fn update(&mut self, context: &mut GameContext) {
		self.game_manager.update(context);		
	}
	
	fn render(&mut self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
		self.game_manager.render(ctx, g, tr)
	}
	
	fn input(&mut self, context: &mut GameContext) {
		self.game_manager.input(context);
	}

	fn dispose(&mut self) {
		self.game_manager.dispose();
	}
	
	fn name(&self) -> &str {
		&"Game"
	}
	
	fn next_scene(&self) -> usize {
		self.scene_token
	}
	
}