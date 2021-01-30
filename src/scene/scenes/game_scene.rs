use crate::util::Load;
use crate::util::text_renderer::TextRenderer;

use crate::game::GameManager;
use crate::scene::Scene;

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

//#[async_trait::async_trait]
impl Load for GameScene {

	fn load(&mut self) {
		self.game_manager.load();
	}

	fn on_start(&mut self) {
		self.game_manager.on_start();
	}
}

impl Scene for GameScene {
	
	fn update(&mut self, delta: f32) {
		self.game_manager.update(delta);	
	}
	
	fn render(&self, tr: &TextRenderer) {
		self.game_manager.render(tr);
	}
	
	fn input(&mut self, delta: f32) {
		self.game_manager.input(delta);
	}

	fn quit(&mut self) {
		self.game_manager.quit();
	}
	
	fn name(&self) -> &str {
		&"Game"
	}
	
	fn next_scene(&self) -> usize {
		self.scene_token
	}
	
}