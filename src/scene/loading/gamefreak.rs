use macroquad::prelude::Color;

use crate::util::graphics::Texture;
use crate::util::graphics::draw_text_left;
use crate::util::graphics::fade_in;
use crate::util::graphics::draw_rect;
use crate::util::graphics::texture::byte_texture;
use firecore_input as input;

use super::LoadingState;

const BACKGROUND_COLOR: Color = macroquad::color_u8!(24, 40, 72, 255);

pub struct GamefreakLoadingScene {
	
    state: LoadingState,
	accumulator: f32,
	rect_size: f32,
	logo_texture: Texture,
	text_texture: Texture,

}

impl GamefreakLoadingScene {
	pub fn new() -> Self {
		Self {
			state: LoadingState::Continue,
			rect_size: 0.0,
			accumulator: 0.0,
			logo_texture: byte_texture(include_bytes!("../../../build/assets/scenes/loading/logo.png")),
			text_texture: byte_texture(include_bytes!("../../../build/assets/scenes/loading/text.png")),
		}
	}
}

impl super::LoadingScene for GamefreakLoadingScene {

	fn on_start(&mut self) {
		self.state = LoadingState::Continue;
		self.accumulator = 0.0;
		self.rect_size = 0.0;
		crate::audio::play_music_named("IntroGamefreak");
	}
	
	fn update(&mut self, delta: f32) {
		self.accumulator += delta;
		if self.rect_size < 96.0 {
			self.rect_size += delta * 120.0;
			if self.rect_size > 96.0 {
				self.rect_size = 96.0;
			}
		}
		if self.accumulator > 8.5 {
			self.state = LoadingState::End
		}
	}
	
	fn render(&self) {
		draw_rect(BACKGROUND_COLOR, 0.0, (crate::HEIGHT_F32 - self.rect_size) / 2.0, 240.0, self.rect_size);
		fade_in(self.logo_texture, 108.0, 45.0, self.accumulator - 6.0, 1.0); //108x, 12y
		fade_in(self.text_texture, 51.0, 74.0, self.accumulator - 4.0, 1.0); //51x, 41y
		draw_text_left(1, &format!("A Button is{:?}Key", input::keyboard::KEY_CONTROLS.read().get(&input::Control::A).unwrap()), 5.0, 5.0);
		draw_text_left(1, &format!("B Button is{:?}Key", input::keyboard::KEY_CONTROLS.read().get(&input::Control::B).unwrap()), 125.0, 5.0);
		draw_text_left(1, "D-Pad is Arrow Keys", 5.0, 15.0);
	}

    fn state(&self) -> &LoadingState {
        &self.state
    }
	
}