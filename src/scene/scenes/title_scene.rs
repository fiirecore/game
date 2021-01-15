use std::time::SystemTime;
use opengl_graphics::GlGraphics;
use piston_window::Context;
use crate::audio::music::Music;
use crate::engine::engine::Texture;
use crate::engine::text::TextRenderer;

use crate::engine::game_context::GameContext;
use crate::io::data::player_data::PlayerData;
use crate::scene::scene::Scene;

use crate::scene::scene::SceneLoad;
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

impl SceneLoad for TitleScene {

	fn load(&mut self, _context: &mut GameContext) {			
		
		self.background_tex = Some(texture_from_path(asset_as_pathbuf("scenes/title/background.png")));
		self.title_tex = Some(texture_from_path(asset_as_pathbuf("scenes/title/static/title.png")));
		self.trademark_tex = Some(texture_from_path(asset_as_pathbuf("scenes/title/trademark.png")));
		self.subtitle_tex = Some(texture_from_path(asset_as_pathbuf("scenes/title/subtitle.png")));
		self.charizard_tex = Some(texture_from_path(asset_as_pathbuf("scenes/title/charizard.png")));
		self.start_tex = Some(texture_from_path(asset_as_pathbuf("scenes/title/start.png")));
		self.loading_tex = Some(texture_from_path(asset_as_pathbuf("scenes/title/loading.png")));
		// music::bind_sound_file(Sound::CryCharizard, asset_as_pathbuf("audio/sound/cry/cry_charizard.aif"));
		// context.load_music(Music::Title, asset_as_pathbuf("audio/music/title.ogg"), PlayableSettings::default());
	}

	fn on_start(&mut self, context: &mut GameContext) {
		self.next = false;
		self.scene_token = 0;
		if cfg!(debug_assertions) && self.skip_on_debug {
			self.next = true;
		} else {
			context.play_music(Music::Title);
			self.start_time = SystemTime::now();
		}
	}

}

impl Scene for TitleScene {
	 
	fn update(&mut self, context: &mut GameContext) {	
		self.counter+=1;
		if self.next && self.rendered /*&& self.end_time.elapsed().unwrap().as_millis() > 1500*/ {
			context.seed_random(self.counter % 256);
			if PlayerData::exists() {
				self.scene_token = 4;
			} else {
				self.scene_token = 6;
			}
		}
		if self.start_time.elapsed().unwrap().as_secs() > 48 {
			self.scene_token = 1;
		}
	}
	
	fn render(&mut self, ctx: &mut Context, g: &mut GlGraphics, _tr: &mut TextRenderer) {
		draw_o(ctx, g, self.background_tex.as_ref(), 0, 0);
		draw_o(ctx, g, self.title_tex.as_ref(), 3, 3);
		draw_o(ctx, g, self.trademark_tex.as_ref(), 158, 53);
		draw_o(ctx, g, self.subtitle_tex.as_ref(), 52, 57);
		if self.start_time.elapsed().unwrap().as_secs() % 2 == 1 {
			draw_o(ctx, g, self.start_tex.as_ref(), 44, 130);
		}
		if self.next {
			draw_o(ctx, g, self.loading_tex.as_ref(), 0, 0);
			self.rendered = true;
		}
		draw_o(ctx, g, self.charizard_tex.as_ref(), 129, 49);
	}
	
	fn input(&mut self, context: &mut GameContext) {
		
		if context.keys[0] == 1 {
			if !self.next {
				// music::play_sound(&Sound::CryCharizard, music::Repeat::Times(0), 0.05);
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