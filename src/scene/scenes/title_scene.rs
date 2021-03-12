use firecore_audio::play_music;
use firecore_input as input;
use crate::util::graphics::Texture;
use crate::scene::Scene;
use crate::util::graphics::texture::byte_texture;
use crate::util::graphics::draw;
use firecore_util::music::Music::Title;

use super::SceneState;

pub struct TitleScene {	
	
	state: SceneState,
	
	accumulator: f32,

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
		    state: SceneState::Continue,
			background_tex: byte_texture(include_bytes!("../../../build/assets/scenes/title/background.png")),		
			title_tex: byte_texture(include_bytes!("../../../build/assets/scenes/title/title.png")),
			trademark_tex: byte_texture(include_bytes!("../../../build/assets/scenes/title/trademark.png")),
			subtitle_tex: byte_texture(include_bytes!("../../../build/assets/scenes/title/subtitle.png")),
			charizard_tex: byte_texture(include_bytes!("../../../build/assets/scenes/title/charizard.png")),
			start_tex: byte_texture(include_bytes!("../../../build/assets/scenes/title/start.png")),
		    accumulator: 0.0,
		}		
	}
	
}

#[async_trait::async_trait(?Send)]
impl Scene for TitleScene {

	async fn load(&mut self) {
		
	}

	async fn on_start(&mut self) {
		self.state = SceneState::Continue;
		play_music(Title);
		self.accumulator = 0.0;
	}
	 
	fn update(&mut self, _delta: f32) {	
		self.accumulator += macroquad::prelude::get_frame_time();
	}
	
	fn render(&self) {
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
		if input::pressed(input::Control::A) {
			macroquad::prelude::rand::srand(self.accumulator as u64 % 256);
			self.state = SceneState::Scene(super::Scenes::MainMenuScene);
		}
	}
	
	fn quit(&mut self) {}
	
	fn state(&self) -> SceneState {
		self.state
	}
	
}