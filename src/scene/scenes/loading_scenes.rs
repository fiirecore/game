use crate::util::graphics::draw_text_left;
use crate::util::graphics::fade_in;
use crate::io::input;
use crate::util::graphics::Texture;
use crate::scene::Scene;
use crate::util::graphics::fade_in_out;
use crate::util::graphics::draw_rect;
use crate::util::graphics::texture::byte_texture;
use super::Scenes;

pub struct LoadingCopyrightScene {
	scene_token: Option<Scenes>,
	accumulator: f32,
	scene_texture: Texture,
}

impl LoadingCopyrightScene {

	pub fn new() -> Self {
		Self {
			scene_token: None,
			scene_texture: byte_texture(include_bytes!("../../../build/assets/scenes/loading/copyright.png")),
			accumulator: 0.0,
		}
	}
	
}

impl Scene for LoadingCopyrightScene {

	fn on_start(&mut self) {
		self.scene_token = None;
		self.accumulator = 0.0;
	}
	
	fn update(&mut self, delta: f32) {
		self.accumulator += delta;
		if self.accumulator > 4.0 {
			self.scene_token = Some(Scenes::TitleScene);
		}
	}
	
	fn render(&self) {
		fade_in_out(self.scene_texture, 0.0, 0.0, self.accumulator, 3.0, 0.5);
	}

	fn input(&mut self, _delta: f32) {
		
	}

	fn quit(&mut self) {
		
	}
	
	fn next_scene(&self) -> Option<Scenes> {
		self.scene_token
	}
	
}

pub struct LoadingGamefreakScene {
	
	scene_token: Option<Scenes>,
	accumulator: f32,
	background_color: [f32; 4],
	logo_texture: Texture,
	text_texture: Texture,

}

impl LoadingGamefreakScene {

	pub fn new() -> LoadingGamefreakScene {

		LoadingGamefreakScene {

			scene_token: None,
			accumulator: 0.0,
			background_color: [24.0/255.0, 40.0/255.0, 72.0/255.0, 1.0],
			logo_texture: byte_texture(include_bytes!("../../../build/assets/scenes/loading/logo.png")),
			text_texture: byte_texture(include_bytes!("../../../build/assets/scenes/loading/text.png")),
		}
	}
	
}

impl Scene for LoadingGamefreakScene {

	fn on_start(&mut self) {
		self.scene_token = None;
		crate::audio::play_music(crate::audio::music::Music::IntroGamefreak);
		self.accumulator = 0.0;
	}
	
	fn update(&mut self, delta: f32) {
		self.accumulator += delta;
		if self.accumulator > 8.5 {
			self.scene_token = Some(Scenes::TitleScene);
		}
	}
	
	fn render(&self) {
		draw_rect(self.background_color, 0.0, 34.0, 240, 96);
		fade_in(self.logo_texture, 108.0, 45.0, self.accumulator - 6.0, 1.0); //108x, 12y
		fade_in(self.text_texture, 51.0, 74.0, self.accumulator - 4.0, 1.0); //51x, 41y
		draw_text_left(1, &format!("A is{:?}Button", input::keyboard::KEY_CONTROLS.read().get(&input::Control::A).unwrap()), 5.0, 5.0);
		draw_text_left(1, &format!("B is{:?}Button", input::keyboard::KEY_CONTROLS.read().get(&input::Control::B).unwrap()), 125.0, 5.0);
		draw_text_left(1, "D-Pad is Arrow Keys", 5.0, 15.0);
		// #[cfg(target_arch = "wasm32")] {
		// 	draw_text_left(1, "The game may stay on a black screen", 5.0, 130.0);
		// 	draw_text_left(1, "while loading.", 5.0, 145.0);
		// }
	}
	
	fn input(&mut self, _delta: f32) { //[ButtonActions; 6]) {
		 if input::pressed(crate::io::input::Control::A) {
			self.scene_token = Some(Scenes::TitleScene);
		 }
	}
	
	fn quit(&mut self) {}
	
	fn next_scene(&self) -> Option<Scenes> {
		self.scene_token
	}
	
}

pub struct LoadingPokemonScene {
	scene_token: Option<Scenes>,
}

impl LoadingPokemonScene {
	pub fn new() -> LoadingPokemonScene {
		LoadingPokemonScene {
			scene_token: None,
		}
	}

}

impl Scene for LoadingPokemonScene {

	fn on_start(&mut self) {
		self.scene_token = Some(Scenes::TitleScene);
	}
	
	fn update(&mut self, _delta: f32) {}
	   
	fn render(&self) {}
	
	fn input(&mut self, _delta: f32) {
		if input::pressed(crate::io::input::Control::A) {
			self.scene_token = Some(Scenes::TitleScene);
		}
	}
	
	fn quit(&mut self) {}
	
	fn next_scene(&self) -> Option<Scenes> {
		self.scene_token
	}
	
}