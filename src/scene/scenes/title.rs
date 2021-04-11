use game::play_music_named;
use game::input;
use game::macroquad::prelude::Texture2D;
use crate::scene::Scene;
use game::graphics::{byte_texture, draw};

use game::scene::{SceneState, Scenes};

pub struct TitleScene {	
	
	state: SceneState,
	
	accumulator: f32,

	background: Texture2D,
	title: Texture2D,
	trademark: Texture2D,
	subtitle: Texture2D,
	charizard: Texture2D,
	start: Texture2D,
	
}

impl Scene for TitleScene {

	fn new() -> TitleScene {
		TitleScene {
		    state: SceneState::Continue,
			background: byte_texture(include_bytes!("../../../build/assets/scenes/title/background.png")),		
			title: byte_texture(include_bytes!("../../../build/assets/scenes/title/title.png")),
			trademark: byte_texture(include_bytes!("../../../build/assets/scenes/title/trademark.png")),
			subtitle: byte_texture(include_bytes!("../../../build/assets/scenes/title/subtitle.png")),
			charizard: byte_texture(include_bytes!("../../../build/assets/scenes/title/charizard.png")),
			start: byte_texture(include_bytes!("../../../build/assets/scenes/title/start.png")),
		    accumulator: 0.0,
		}		
	}

	fn on_start(&mut self) {
		self.state = SceneState::Continue;
		play_music_named("Title");
		self.accumulator = 0.0;
	}
	 
	fn update(&mut self, delta: f32) {	
		self.accumulator += delta;
	}
	
	fn render(&self) {
		draw(self.background, 0.0, 0.0);
		draw(self.title, 3.0, 3.0);
		draw(self.trademark, 158.0, 53.0);
		draw(self.subtitle, 52.0, 57.0);
		if self.accumulator as u8 % 2 == 1 {
			draw(self.start, 44.0, 130.0);
		}
		draw(self.charizard, 129.0, 49.0);
	}
	
	fn input(&mut self, _delta: f32) {
		if input::pressed(input::Control::A) {
			let seed = self.accumulator as u64 % 256;
			crate::util::seed_randoms(seed);
			self.state = SceneState::Scene(Scenes::MainMenu);
		}
	}
	
	fn quit(&mut self) {
		self.state = SceneState::Continue;
	}
	
	fn state(&self) -> SceneState {
		self.state
	}
	
}