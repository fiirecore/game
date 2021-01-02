use std::time::SystemTime;
use opengl_graphics::GlGraphics;
use piston_window::Context;
use crate::engine::engine::Texture;
use crate::engine::text::TextRenderer;

use crate::engine::game_context::GameContext;
use crate::scene::scene::Scene;
use crate::audio::{music::Music, sound::Sound};

use crate::util::file_util::asset_as_pathbuf;
use crate::util::texture_util::texture_from_path;
use crate::util::render_util::draw_o;

pub struct TitleScene {	
	
	scene_token: usize,
	skip_on_debug: bool,
	
	next: bool,
	start_time: SystemTime,
	//end_time: SystemTime,
	counter: u64,
	rendered: bool,

	background_tex: Option<Texture>, //TO-DO: change to 3 (5 including black) seperate solid color textures
	title_tex: Option<Texture>,
	trademark_tex: Option<Texture>,
	subtitle_tex: Option<Texture>,
	charizard_tex: Option<Texture>,
	start_tex: Option<Texture>,
	loading_tex: Option<Texture>,
	
}


impl TitleScene {	
	
	pub fn new() -> TitleScene {
		TitleScene {
			
			scene_token: 0,
			skip_on_debug: true,

			start_time: SystemTime::now(),
			//end_time: SystemTime::now(),
			next: false,
			counter: 0,
			rendered: false,
			
			background_tex: None,
			title_tex: None,
			trademark_tex: None,
			subtitle_tex: None,
			charizard_tex: None,
			start_tex: None,
			loading_tex: None,
			
		}		
	}
	
}

impl Scene for TitleScene {

	fn load(&mut self) {			
		
		self.background_tex = Some(texture_from_path(asset_as_pathbuf("scenes/title/background.png")));
		self.title_tex = Some(texture_from_path(asset_as_pathbuf("scenes/title/static/title.png")));
		self.trademark_tex = Some(texture_from_path(asset_as_pathbuf("scenes/title/trademark.png")));
		self.subtitle_tex = Some(texture_from_path(asset_as_pathbuf("scenes/title/subtitle.png")));
		self.charizard_tex = Some(texture_from_path(asset_as_pathbuf("scenes/title/charizard.png")));
		self.start_tex = Some(texture_from_path(asset_as_pathbuf("scenes/title/start.png")));
		self.loading_tex = Some(texture_from_path(asset_as_pathbuf("scenes/title/loading.png")));
		music::bind_sound_file(Sound::CryCharizard, asset_as_pathbuf("audio/sound/cry/cry_charizard.aif"));
		music::bind_music_file(Music::Title, asset_as_pathbuf("audio/music/mus_title.mid"))

	}

	fn on_start(&mut self, _context: &mut GameContext) {
		self.next = false;
		self.scene_token = 0;
		if cfg!(debug_assertions) && self.skip_on_debug {
			self.next = true;
		} else {
			music::set_volume(0.2);
			music::play_music(&Music::Title, music::Repeat::Times(0));
			self.start_time = SystemTime::now();
		}
	}
	 
	fn update(&mut self, context: &mut GameContext) {	
		self.counter+=1;
		if self.next && self.rendered /*&& self.end_time.elapsed().unwrap().as_millis() > 1500*/ {
			context.seed_random(self.counter % 256);
			self.scene_token = 9;
		}
		if self.start_time.elapsed().unwrap().as_secs() > 48 {
			self.scene_token = 1;
		}
	}
	
	fn render(&mut self, ctx: &mut Context, g: &mut GlGraphics, _tr: &mut TextRenderer) {
		//if self.next  {
		//	fade_out_o(c, gl, &self.background_tex, 0, 0, self.end_time, 1250, 500, 255, 255, 255);
		//	fade_out_o(c, gl, &self.title_tex, 3, 3, self.end_time, 1250, 500, 255, 255, 255);
		//	fade_out_o(c, gl, &self.trademark_tex, 158, 53, self.end_time, 1250, 500, 255, 255, 255);
		//	fade_out_o(c, gl, &self.subtitle_tex, 52, 57, self.end_time, 1250, 500, 255, 255, 255);
		//	fade_out_o(c, gl, &self.charizard_tex, 129, 49, self.end_time, 1250, 500, 255, 255, 255);
		//	if self.start_time.elapsed().unwrap().as_secs() % 2 == 1 {
		//		fade_out_o(c, gl, &self.start_tex, 44, 130, self.end_time, 1250, 500, 255, 255, 255);
		//	} else {
		//		
		//	}
		//} else {
			draw_o(ctx, g, &self.background_tex, 0, 0);
			draw_o(ctx, g, &self.title_tex, 3, 3);
			draw_o(ctx, g, &self.trademark_tex, 158, 53);
			draw_o(ctx, g, &self.subtitle_tex, 52, 57);
			if self.start_time.elapsed().unwrap().as_secs() % 2 == 1 {
				draw_o(ctx, g, &self.start_tex, 44, 130);
			}
			if self.next {
				draw_o(ctx, g, &self.loading_tex, 0, 0);
				self.rendered = true;
			}
			draw_o(ctx, g, &self.charizard_tex, 129, 49);
		//}
	}
	
	fn input(&mut self, context: &mut GameContext) {
		
		if context.keys[0] == 1 {
			if !self.next {
				music::play_sound(&Sound::CryCharizard, music::Repeat::Times(0), 0.05);
				//self.end_time = SystemTime::now();
			}
			self.next = true;
		}
		
	}
	
	fn dispose(&mut self) {}
	
	fn name(&self) -> &str {
		"Title"
	}
	
	fn next_scene(&self) -> usize {
		self.scene_token
	}
	
}