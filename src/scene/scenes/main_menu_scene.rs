use crate::scene::Scene;
use super::Scenes;

pub struct MainMenuScene {
	scene_token: Option<Scenes>,
}

impl MainMenuScene {

	pub fn new() -> MainMenuScene {
		MainMenuScene {
			scene_token: None,
		}
	}

}

impl Scene for MainMenuScene {

	// have normal main menu + video settings + controls + exit

	fn on_start(&mut self) {
		self.scene_token = Some(Scenes::GameScene);
	}
	
	fn update(&mut self, _delta: f32) {}
	
	fn render(&self) {}
	
	fn input(&mut self, _delta: f32) {}
	
	fn quit(&mut self) {}
	
	fn next_scene(&self) -> Option<Scenes> {
		self.scene_token
	}
	
}