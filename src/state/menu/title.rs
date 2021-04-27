use game::{
	play_music_named,
	input::{pressed, Control},
	macroquad::prelude::Texture2D,
	graphics::{byte_texture, draw},
};

use super::{MenuState, MenuStateAction, MenuStates};

pub struct TitleState {	
	
	action: Option<MenuStateAction>,
	
	accumulator: f32,

	background: Texture2D,
	title: Texture2D,
	trademark: Texture2D,
	subtitle: Texture2D,
	charizard: Texture2D,
	start: Texture2D,
	
}

impl MenuState for TitleState {

	fn new() -> Self {
		Self {
		    action: None,
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
		if pressed(Control::A) {
			let seed = self.accumulator as u64 % 256;
			crate::seed_randoms(seed);
			self.action = Some(MenuStateAction::Goto(MenuStates::MainMenu));
		}
	}
	
	fn quit(&mut self) {
		
	}
	
	fn action(&mut self) -> &mut Option<MenuStateAction> {
		&mut self.action
	}
	
}