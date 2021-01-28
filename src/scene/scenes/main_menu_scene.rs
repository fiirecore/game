use crate::util::Load;
use crate::util::text_renderer::TextRenderer;
use crate::scene::Scene;

pub struct MainMenuScene {
	scene_token: usize,
}

impl MainMenuScene {

	pub fn new() -> MainMenuScene {

		MainMenuScene {

			scene_token: 0,

		}

	}

}

////#[async_trait::async_trait]
impl Load for MainMenuScene {

	fn load(&mut self) {

	}

	fn on_start(&mut self) {
		self.scene_token = 5;
	}

}

impl Scene for MainMenuScene {

	// have normal main menu + video settings + controls + exit
	
	fn update(&mut self, _delta: f32) {}
	
	fn render(&self, _tr: &TextRenderer) {}
	
	fn input(&mut self, _delta: f32) {}
	
	fn quit(&mut self) {}
	
	fn name(&self) -> &str {
		&"Main Menu"
	}
	
	fn next_scene(&self) -> usize {
		self.scene_token
	}
	
}