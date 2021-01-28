use crate::util::Load;
use crate::scene::Scene;

pub struct CharacterCreationScene {
	scene_token: usize,
}

impl CharacterCreationScene {
	pub fn new() -> CharacterCreationScene {
		CharacterCreationScene {
			scene_token: 1,
		}
	}
}

//#[async_trait::async_trait]
impl Load for CharacterCreationScene {
	
    fn load(&mut self) {
        
    }

    fn on_start(&mut self) {
        
    }
}

impl Scene for CharacterCreationScene {
	
	fn update(&mut self, _delta: f32) {}
	
	fn render(&self, _tr: &crate::util::text_renderer::TextRenderer) {}
	
	fn input(&mut self, _delta: f32) {}
	
	fn quit(&mut self) {}
	
	fn name(&self) -> &str {
		&"Character Creation"
	}
	
	fn next_scene(&self) -> usize {
		self.scene_token
	}
	
}