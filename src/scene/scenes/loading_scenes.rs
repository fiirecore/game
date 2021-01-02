use std::time::SystemTime;
use opengl_graphics::GlGraphics;
use piston_window::Context;

use crate::{audio::music::Music, engine::engine::Texture};
use crate::engine::text::TextRenderer;

use crate::engine::game_context::GameContext;
use crate::scene::scene::Scene;
//use crate::audio::sound_engine::{Source, SoundSource};

use crate::util::render_util::fade_in_out_o;
use crate::util::render_util::draw_rect;
use crate::util::file_util::asset_as_pathbuf;
use crate::util::texture_util::texture_from_path;

pub struct LoadingCopyrightScene {
	scene_token: usize,
	started: bool,
	start_time: SystemTime,
	scene_texture: Option<Texture>,
}

impl LoadingCopyrightScene {
	
	pub fn new() -> LoadingCopyrightScene {
		
		LoadingCopyrightScene {
			
			scene_token: 0,
			started: false,
			start_time: SystemTime::now(),	
			scene_texture: None,
			
		}
	}
	
	fn next(&mut self) {
		self.scene_token = 2;
	}
	
}

impl Scene for LoadingCopyrightScene {
	
	fn load(&mut self) {
		if !cfg!(debug_assertions) {
			self.scene_texture = Some(texture_from_path(asset_as_pathbuf("scenes/loading/copyright.png")));
		}		
	}

	fn on_start(&mut self, _context: &mut GameContext) {
		self.scene_token = 0;
		if cfg!(debug_assertions) {
			self.next();
		}
		self.start_time = SystemTime::now();
	}
	
	fn update(&mut self, _context: &mut GameContext) {
		if !self.started {
			self.start_time = SystemTime::now();
			self.started = true;
		}
		if self.start_time.elapsed().unwrap().as_secs() > 3 {
			self.next();
		}
	}
	
	fn render(&mut self, ctx: &mut Context, g: &mut GlGraphics, _tr: &mut TextRenderer) {
		fade_in_out_o(ctx, g, &self.scene_texture, 0, 0, self.start_time, 2500, 500, 255, 255, 255);
//		draw_o(ctx, g, &self.scene_texture, 0, 0);
	}
	
	fn dispose(&mut self) {

	}
	
	fn name(&self) -> &str {
		&"Loading - Copyright"
	}
	
	fn next_scene(&self) -> usize {self.scene_token}
	
}

pub struct LoadingGamefreakScene {
	
	scene_token: usize,
	start_time: SystemTime,
	background_color: [f32; 4],

//	sound: Source,

}

impl LoadingGamefreakScene {

	pub fn new() -> LoadingGamefreakScene {

		LoadingGamefreakScene {

			scene_token: 0,
			start_time: SystemTime::now(),
			background_color: [24.0/255.0, 40.0/255.0, 72.0/255.0, 1.0],

//			sound: ,

		}
	}	

	fn next(&mut self) {
		self.scene_token = 3;
	}
	
}

impl Scene for LoadingGamefreakScene {
	
	fn load(&mut self) {
		if !cfg!(debug_assertions) {
			music::bind_music_file(Music::IntroGamefreak, asset_as_pathbuf("audio/music/mus_game_freak.mid"));
		}
		
	}

	fn on_start(&mut self, _context: &mut GameContext) {
		self.scene_token = 0;
//		use crate::engine::audio::{Source, SoundSource};
//		let mut sound = Source::new(_context,asset_as_pathbuf("audio/music/intro_gamefreak.mp3")).unwrap();
//		sound.play();
		if !cfg!(debug_assertions) {
			//context.audio_context.play(1);
			music::play_music(&Music::IntroGamefreak, music::Repeat::Times(0));
		} else {
			self.next();
		}
		self.start_time = SystemTime::now();
	}
	
	fn update(&mut self, _context: &mut GameContext) {
		if self.start_time.elapsed().unwrap().as_millis() > 8500 {
			self.next();
		}
	}
	
	fn render(&mut self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
		draw_rect(ctx, g, self.background_color.into(), 0, 34, 240, 96);
		tr.render_text_from_left(ctx, g, 1, "X is A Button", 5, 34);
		tr.render_text_from_left(ctx, g, 1, "Z is B button", 5, 49);
		tr.render_text_from_left(ctx, g, 1, "D-Pad is Arrow Keys", 5, 64);
		tr.render_text_from_left(ctx, g, 1, "F1 to battle", 5, 79);
		tr.render_text_from_left(ctx, g, 1, "F2 to toggle noclip", 5, 94);
		tr.render_text_from_left(ctx, g, 1, "F3 to toggle console", 5, 109);
	}
	
	fn input(&mut self, context: &mut GameContext) { //[ButtonActions; 6]) {
		 if context.keys[1] == 1 {
			 self.next();
		 }
	}
	
	fn dispose(&mut self) {}
	
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
}

impl Scene for LoadingPokemonScene {
	
	//fn load(&mut self, _ctx: &mut Context, _context: &mut GameContext) {}

	fn on_start(&mut self, _context: &mut GameContext) {
		self.scene_token = 4;
	}
	
	//fn update(&mut self, _ctx: &mut Context, _context: &mut GameContext) {}
	   
	//fn render(&mut self, _ctx: &mut Context, _tr: &mut TextRenderer) {}
	
	//fn input(&mut self, context: &mut GameContext) { //[ButtonActions; 6]) {
	//	if context.keys[1] == 1 {
	//		self.scene_token = 4;
	//	}
	//}
	
	//fn dispose(&mut self) {}
	
	fn name(&self) -> &str {
		&"Loading - Pokemon Intro"
	}
	
	fn next_scene(&self) -> usize {self.scene_token}
	
}