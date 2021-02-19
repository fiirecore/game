use crate::util::Load;


use crate::game::GameManager;
use crate::scene::Scene;

pub struct GameScene {

	scene_token: usize,
	
	game_manager: GameManager,

}

impl GameScene {
	
	pub async fn new() -> GameScene {

		GameScene {
			
			scene_token: 0,

			game_manager: GameManager::new().await,

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
	
	fn render(&self) {
		self.game_manager.render();
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