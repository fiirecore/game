use crate::engine::game_context::GameContext;
use crate::scene::scene::Scene;
use crate::scene::scene::SceneLoad;

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

// #[async_trait::async_trait]
impl SceneLoad for CharacterCreationScene {
	
    fn load(&mut self, _context: &mut GameContext) {
        
    }

    fn on_start(&mut self, _context: &mut GameContext) {
        
    }
}

impl Scene for CharacterCreationScene {

//	fn load(&mut self, _ctx: &mut Context, _context: &mut GameContext) {}
	
//	fn update(&mut self, _ctx: &mut Context, _context: &mut GameContext) {}
	
//	fn render(&mut self, _ctx: &mut Context, _gc: &mut TextRenderer) {}
	
//	fn input(&mut self, _context: &mut GameContext) {}
	
//	fn dispose(&mut self) {}
	
	fn name(&self) -> &str {
		&"Character Creation"
	}
	
	fn next_scene(&self) -> usize {
		self.scene_token
	}
	
}