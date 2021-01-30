use kira::sound::handle::SoundHandle;

use crate::util::Load;
use crate::util::input;
use crate::util::texture::Texture;
use crate::util::text_renderer::TextRenderer;
//use async_trait::async_trait;
use crate::scene::Scene;
use crate::util::render::fade_in_out;
use crate::util::render::draw_rect;
use crate::util::texture::byte_texture;

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
	
	fn render(&self, _tr: &TextRenderer) {
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
	sound: Option<SoundHandle>,
	length: f32,

}

impl LoadingGamefreakScene {

	pub fn new(sound: Option<SoundHandle>) -> LoadingGamefreakScene {

		let def = 8.5;
		let length: f32 = match sound {
		    Some(ref sound) => {
				match sound.semantic_duration() {
				    Some(len) => {
						len as _
					}
				    None => {
						def
					}
				}
			}
		    None => {
				def
			}
		};

		LoadingGamefreakScene {

			scene_token: 0,
			accumulator: 0.0,
			background_color: [24.0/255.0, 40.0/255.0, 72.0/255.0, 1.0],
			sound: sound,
			length: length,

		}
	}	

	pub fn render_notr(&self) {
		draw_rect(self.background_color, 0.0, 34.0, 240, 96);
	}
	
}

//#[async_trait]
impl Load for LoadingGamefreakScene {

	fn load(&mut self) {

	}

	fn on_start(&mut self) {
		self.scene_token = 0;
		if let Some(ref mut sound) = self.sound {
			if let Err(err) = sound.play(kira::instance::InstanceSettings::default()) {
				macroquad::prelude::warn!("Error playing sound: {}", err);
			}
		}
		self.accumulator = 0.0;
	}

}

impl Scene for LoadingGamefreakScene {
	
	fn update(&mut self, delta: f32) {
		self.accumulator += delta;
		if self.accumulator > self.length {
			self.scene_token = 2;
		}
	}
	
	fn render(&self, _tr: &TextRenderer) {
		self.render_notr();
		// tr.render_text_from_left(1, "X is A Button", 5.0, 34.0);
		// tr.render_text_from_left(1, "Z is B button", 5.0, 49.0);
		// tr.render_text_from_left(1, "D-Pad is Arrow Keys", 5.0, 64.0);
		// tr.render_text_from_left(1, "F1 to battle", 5.0, 79.0);
		// tr.render_text_from_left(1, "F2 to toggle noclip", 5.0, 94.0);
		// tr.render_text_from_left(1, "F3 to toggle console", 5.0, 109.0);
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
	   
	fn render(&self, _tr: &TextRenderer) {
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