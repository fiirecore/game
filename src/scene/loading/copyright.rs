
use firecore_util::text::TextColor;
use macroquad::prelude::Texture2D;

use crate::util::graphics::{byte_texture, fade_in_out};

use super::LoadingState;

pub struct CopyrightLoadingScene {
	state: LoadingState,
	accumulator: f32,
	scene_texture: Texture2D,
}

impl CopyrightLoadingScene {

	pub fn new() -> Self {
		Self {
			state: LoadingState::Continue,
			scene_texture: byte_texture(include_bytes!("../../../build/assets/scenes/loading/copyright.png")),
			accumulator: 0.0,
		}
	}
	
}

impl super::LoadingScene for CopyrightLoadingScene {

	fn on_start(&mut self) {
		self.state = LoadingState::Continue;
		self.accumulator = 0.0;
	}
	
	fn update(&mut self, delta: f32) {
		self.accumulator += delta;
		if self.accumulator > 4.0 {
			self.state = LoadingState::Scene(super::LoadingScenes::Gamefreak);
		}
	}
	
	fn render(&self) {
		fade_in_out(self.scene_texture, 0.0, 0.0, self.accumulator, 3.0, 0.5);
		crate::util::graphics::draw_text_left(1, &format!("v{}", crate::VERSION), TextColor::White, 2.0, 0.0);
	}

    fn state(&self) -> &LoadingState {
        &self.state
    }
	
}