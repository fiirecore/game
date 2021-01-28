use crate::util::Load;
use crate::util::input;
use crate::util::texture::Texture;
use crate::util::text_renderer::TextRenderer;
use crate::io::data::player_data::PlayerData;
use crate::scene::Scene;
use crate::util::texture::byte_texture;
use crate::util::render::draw;

pub struct TitleScene {	
	
	scene_token: usize,
	skip_on_debug: bool,
	
	next: bool,
	accumulator: f32,
	counter: u64,

	background_tex: Texture, //TO-DO: change to 3 (5 including black) seperate solid color textures
	title_tex: Texture,
	trademark_tex: Texture,
	subtitle_tex: Texture,
	charizard_tex: Texture,
	start_tex: Texture,
	
}


impl TitleScene {	
	
	pub fn new() -> TitleScene {
		TitleScene {
			skip_on_debug: true,
			background_tex: byte_texture(include_bytes!("../../../include/scenes/title/background.png")),		
			title_tex: byte_texture(include_bytes!("../../../include/scenes/title/title.png")),
			trademark_tex: byte_texture(include_bytes!("../../../include/scenes/title/trademark.png")),
			subtitle_tex: byte_texture(include_bytes!("../../../include/scenes/title/subtitle.png")),
			charizard_tex: byte_texture(include_bytes!("../../../include/scenes/title/charizard.png")),
			start_tex: byte_texture(include_bytes!("../../../include/scenes/title/start.png")),
		    scene_token: 0,
		    next: false,
		    accumulator: 0.0,
		    counter: 0, 
		}		
	}
	
}

//#[async_trait::async_trait]
impl Load for TitleScene {

	fn load(&mut self) {}

	fn on_start(&mut self) {
		self.next = false;
		self.scene_token = 0;
		if !crate::not_debug() && self.skip_on_debug {
			self.next = true;
		} else {
			//context.play_music(Music::Title);
			self.accumulator = 0.0;
		}
	}

}

impl Scene for TitleScene {
	 
	fn update(&mut self, _delta: f32) {	
		self.accumulator += macroquad::prelude::get_frame_time();
		self.counter+=1;
		if self.next {
			macroquad::prelude::rand::srand(self.counter % 256);
			if PlayerData::exists() {
				self.scene_token = 2;
			} else {
				self.scene_token = 2;//4;
			}
		}
		if self.accumulator > 48.0 {
			self.scene_token = 1;
		}
	}
	
	fn render(&self, _tr: &TextRenderer) {
		draw(self.background_tex, 0.0, 0.0);
		draw(self.title_tex, 3.0, 3.0);
		draw(self.trademark_tex, 158.0, 53.0);
		draw(self.subtitle_tex, 52.0, 57.0);
		if self.accumulator as u8 % 2 == 1 {
			draw(self.start_tex, 44.0, 130.0);
		}
		draw(self.charizard_tex, 129.0, 49.0);
	}
	
	fn input(&mut self, _delta: f32) {
		
		if input::pressed(crate::util::input::Control::A) {
			if !self.next {
				// music::play_sound(&Sound::CryCharizard, music::Repeat::Times(0), 0.05);
				//self.end_time = Instant::now();
			}
			self.next = true;
		}
		
	}
	
	fn quit(&mut self) {}
	
	fn name(&self) -> &str {
		"Title"
	}
	
	fn next_scene(&self) -> usize {
		self.scene_token
	}
	
}