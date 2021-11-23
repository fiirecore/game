
use game::{
	text::TextColor,
	graphics::{byte_texture, fade_in_out, draw_text_left},
	macroquad::prelude::Texture2D,
};

use super::{LoadingState, LoadingScenes};

pub struct CopyrightLoadingScene {
	state: LoadingState,
	accumulator: f32,
	scene_texture: Texture2D,
}

impl super::LoadingScene for CopyrightLoadingScene {

	fn new() -> Self {
		Self {
			state: LoadingState::Continue,
			scene_texture: Texture::new(include_bytes!("../../../build/assets/scenes/loading/copyright.png")),
			accumulator: 0.0,
		}
	}

	fn on_start(&mut self) {
		self.state = LoadingState::Continue;
		self.accumulator = 0.0;
	}
	
	fn update(&mut self, delta: f32) {
		self.accumulator += delta;
		if self.accumulator > 4.0 {
			self.state = LoadingState::Scene(LoadingScenes::Gamefreak);
		}
	}
	
	fn render(&self) {
		fade_in_out(self.scene_texture, 0.0, 0.0, self.accumulator, 3.0, 0.5);
		draw_text_left(1, &format!("v{}", crate::VERSION), &TextColor::White, 2.0, 0.0);
	}

    fn state(&self) -> LoadingState {
    	self.state
    }
	
}