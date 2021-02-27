use crate::scene::Scene;
use super::SceneState;
use super::Scenes;

pub struct MainMenuScene {
	state: SceneState,
}

impl MainMenuScene {

	pub fn new() -> MainMenuScene {
		MainMenuScene {
			state: SceneState::Scene(Scenes::GameScene),
		}
	}

}

impl Scene for MainMenuScene {

	// have normal main menu + video settings + controls + exit

	fn on_start(&mut self) {}
	
	fn update(&mut self, _delta: f32) {}
	
	fn render(&self) {}
	
	fn input(&mut self, _delta: f32) {}
	
	fn quit(&mut self) {}
	
	fn state(&self) -> SceneState {
		self.state
	}
	
}