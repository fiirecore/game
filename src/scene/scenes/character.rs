use firecore_data::get_mut;
use firecore_data::player::PlayerSaves;

use crate::scene::Scene;
use super::{SceneState, Scenes};

pub struct CharacterCreationScene {
	state: SceneState,
}

impl Scene for CharacterCreationScene {

	fn new() -> Self {
		Self {
			state: SceneState::Continue,
		}
	}
	
	fn on_start(&mut self) {
		if let Some(mut saves) = get_mut::<PlayerSaves>() {
			saves.select_new(&format!("Player{}", macroquad::miniquad::date::now() as u64 % 1000000));
		} else {
			panic!("Could not get player data!");
		}
		self.state = SceneState::Scene(Scenes::MainMenu);
	}
	
	fn input(&mut self, _delta: f32) {
		
	}
	
	fn update(&mut self, _delta: f32) {
		
	}

	fn render(&self) {}

	fn quit(&mut self) {
		self.state = SceneState::Continue;
	}

	fn state(&self) -> SceneState {
        self.state
    }
}