use crate::util::Load;
use crate::util::graphics::fade_in;
use crate::util::input;
use crate::util::graphics::Texture;

//use async_trait::async_trait;
use crate::scene::Scene;
use crate::util::graphics::fade_in_out;
use crate::util::graphics::draw_rect;
use crate::util::graphics::texture::byte_texture;

pub struct LoadingCopyrightScene {
	scene_token: usize,
	accumulator: f32,
	scene_texture: Texture,
}

impl LoadingCopyrightScene {

	pub fn new() -> Self {
		Self {
			scene_texture: byte_texture(include_bytes!("../../../build/assets/scenes/loading/copyright.png")),
			accumulator: 0.0,
			scene_token: 0,
		}
	}

	pub fn render_notr(&self) {
		fade_in_out(self.scene_texture, 0.0, 0.0, self.accumulator, 3.0, 0.5);
	}
	
}

impl Load for LoadingCopyrightScene {

	fn load(&mut self) {}

	fn on_start(&mut self) {
		self.scene_token = 0;
		self.accumulator = 0.0;
	}

}

impl Scene for LoadingCopyrightScene {
	
	fn update(&mut self, delta: f32) {
		self.accumulator += delta;
		if self.accumulator > 4.0 {
			self.scene_token = 1;
		}
	}
	
	fn render(&self) {
		self.render_notr();
	}

	fn input(&mut self, _delta: f32) {
		
	}

	fn quit(&mut self) {
		
	}
	
	fn name(&self) -> &str {
		&"Loading - Copyright"
	}
	
	fn next_scene(&self) -> usize {self.scene_token}
	
}

pub struct LoadingGamefreakScene {
	
	scene_token: usize,
	accumulator: f32,
	background_color: [f32; 4],
	logo_texture: Texture,
	text_texture: Texture,

}

impl LoadingGamefreakScene {

	pub fn new() -> LoadingGamefreakScene {

		LoadingGamefreakScene {

			scene_token: 0,
			accumulator: 0.0,
			background_color: [24.0/255.0, 40.0/255.0, 72.0/255.0, 1.0],
			logo_texture: byte_texture(include_bytes!("../../../build/assets/scenes/loading/logo.png")),
			text_texture: byte_texture(include_bytes!("../../../build/assets/scenes/loading/text.png")),
		}
	}	

	pub fn render_notr(&self) {
		draw_rect(self.background_color, 0.0, 34.0, 240, 96);
		fade_in(self.logo_texture, 108.0, 45.0, self.accumulator - 6.0, 1.0); //108x, 12y
		fade_in(self.text_texture, 51.0, 74.0, self.accumulator - 4.0, 1.0); //51x, 41y
	}
	
}

//#[async_trait]
impl Load for LoadingGamefreakScene {

	fn load(&mut self) {

	}

	fn on_start(&mut self) {
		self.scene_token = 0;
		crate::audio::play_music(crate::audio::music::Music::IntroGamefreak);
		self.accumulator = 0.0;
	}

}

impl Scene for LoadingGamefreakScene {
	
	fn update(&mut self, delta: f32) {
		self.accumulator += delta;
		if self.accumulator > 8.5 {
			self.scene_token = 2;
		}
	}
	
	fn render(&self) {
		self.render_notr();
		// draw_text_left(1, "X is A Button", 5.0, 34.0);
		// draw_text_left(1, "Z is B button", 5.0, 49.0);
		// draw_text_left(1, "D-Pad is Arrow Keys", 5.0, 64.0);
		// draw_text_left(1, "F1 to battle", 5.0, 79.0);
		// draw_text_left(1, "F2 to toggle noclip", 5.0, 94.0);
		// draw_text_left(1, "F3 to toggle console", 5.0, 109.0);
	}
	
	fn input(&mut self, _delta: f32) { //[ButtonActions; 6]) {
		 if input::pressed(crate::util::input::Control::A) {
			self.scene_token = 2;
		 }
	}
	
	fn quit(&mut self) {}
	
	fn name(&self) -> &str {
		&"Loading - Gamefreak Intro"
	}
	
	fn next_scene(&self) -> usize {self.scene_token}
	
}

pub struct LoadingPokemonScene {
	scene_token: usize,
}

impl LoadingPokemonScene {
	pub fn new() -> LoadingPokemonScene {
		LoadingPokemonScene {
			scene_token: 0,
		}
	}

	pub fn render_notr(&self) {

	}
}

//#[async_trait]
impl Load for LoadingPokemonScene {

	fn load(&mut self) {}

	fn on_start(&mut self) {
		self.scene_token = 3;
	}
	
}

impl Scene for LoadingPokemonScene {
	
	fn update(&mut self, _delta: f32) {}
	   
	fn render(&self) {
		self.render_notr();
	}
	
	fn input(&mut self, _delta: f32) { //[ButtonActions; 6]) {
		if input::pressed(crate::util::input::Control::B) {
			self.scene_token = 4;
		}
	}
	
	fn quit(&mut self) {}
	
	fn name(&self) -> &str {
		&"Loading - Pokemon Intro"
	}
	
	fn next_scene(&self) -> usize {self.scene_token}
	
}