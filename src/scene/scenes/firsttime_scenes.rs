use crate::util::Load;
use crate::util::input;
use crate::util::texture::Texture;
use crate::scene::Scene;
use crate::util::text_renderer::TextRenderer;

use crate::util::file::asset_as_pathbuf;
use crate::util::render::draw;
use crate::util::texture::load_texture;
//use async_trait::async_trait;
pub struct FirstTimeControlsScene {
	scene_token: usize,
	page: u8,
	background_tex: Texture,
	dpad_b_tex: Option<Texture>,
	a_b_tex: Option<Texture>,
	b_b_tex: Option<Texture>,
	start_b_tex: Option<Texture>,
	select_b_tex: Option<Texture>,
	lr_b_tex: Option<Texture>,
}

impl FirstTimeControlsScene {
	
	pub async fn new() -> FirstTimeControlsScene {
		FirstTimeControlsScene {
			scene_token: 0,
			page: 0,
			background_tex: load_texture(asset_as_pathbuf("scenes/firsttime/controls/background.png")).await,
			dpad_b_tex: None,
			a_b_tex: None,
			b_b_tex: None,
			start_b_tex: None,
			select_b_tex: None,
			lr_b_tex: None,
		}
	}
	
}

//#[async_trait]
impl Load for FirstTimeControlsScene {

	fn load(&mut self) {
		
		self.page = 0;
		self.dpad_b_tex = None;
		self.a_b_tex = None;
		self.b_b_tex = None;
		self.start_b_tex = None;
		self.select_b_tex = None;
		self.lr_b_tex = None;
		
	}

	fn on_start(&mut self) {
		
	}

}

impl Scene for FirstTimeControlsScene {
	
	fn update(&mut self, _delta: f32) {}
	
	fn render(&self) {
		draw(self.background_tex, 0.0, 0.0);
	}
	
	fn input(&mut self, _delta: f32) { //[ButtonActions; 6]) {
		if input::pressed(crate::util::input::Control::A) { //ButtonActions::PRESSED {
			
			self.scene_token = 8;
		}
	}
	
	fn quit(&mut self) {}
	
	fn name(&self) -> &str {
		&"First Time - Controls"
	}
	
	fn next_scene(&self) -> usize {self.scene_token}
	
}

//firsttime narrative scene here
pub struct FirstTimeNarrativeScene {
	
	scene_token: usize,
	page: u8,
	background_tex: Texture,
	pikachu_tex: Option<Texture>, // change to pikachu sprite

}

impl FirstTimeNarrativeScene {
	
	pub async fn new() -> FirstTimeNarrativeScene {
		FirstTimeNarrativeScene {
			scene_token: 0,
			page: 0,
			background_tex: load_texture(asset_as_pathbuf("firsttime/narrative/background.png")).await,
			pikachu_tex: None,
		}
	}
	
}

//#[async_trait]
impl Load for FirstTimeNarrativeScene {

	fn load(&mut self) {
		
		self.page = 0;
		self.pikachu_tex = None;
		
	}

	fn on_start(&mut self) {
		
	}

}

impl Scene for FirstTimeNarrativeScene {
	
	fn update(&mut self, _delta: f32) {}
	
	fn render(&self) {
		draw(self.background_tex, 0.0, 0.0);
	}
	
	fn input(&mut self, _delta: f32) { //[ButtonActions; 6]) {
		if input::pressed(crate::util::input::Control::A) { //ButtonActions::PRESSED {
			
			self.scene_token = 6;
		}
	}
	
	fn quit(&mut self) {}
	
	fn name(&self) -> &str {
		&"First Time - Narrative"
	}
	
	fn next_scene(&self) -> usize {self.scene_token}
	
}