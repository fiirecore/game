
use opengl_graphics::Texture;
use crate::scene::Scene;
use crate::scene::SceneLoad;
use opengl_graphics::GlGraphics;
use piston_window::Context;
use crate::util::text_renderer::TextRenderer;

use crate::util::context::GameContext;

use crate::util::file::asset_as_pathbuf;
use crate::util::texture_util::texture_from_path;
use crate::util::render_util::draw_o;

pub struct FirstTimeControlsScene {
	scene_token: usize,
	page: u8,
	background_tex: Option<Texture>,
	dpad_b_tex: Option<Texture>,
	a_b_tex: Option<Texture>,
	b_b_tex: Option<Texture>,
	start_b_tex: Option<Texture>,
	select_b_tex: Option<Texture>,
	lr_b_tex: Option<Texture>,
}

impl FirstTimeControlsScene {
	
	pub fn new() -> FirstTimeControlsScene {
		FirstTimeControlsScene {
			scene_token: 0,
			page: 0,
			background_tex: None,
			dpad_b_tex: None,
			a_b_tex: None,
			b_b_tex: None,
			start_b_tex: None,
			select_b_tex: None,
			lr_b_tex: None,
		}
	}
	
}


impl SceneLoad for FirstTimeControlsScene {

	fn load(&mut self, _context: &mut GameContext) {
		
		self.page = 0;
		self.background_tex = Some(texture_from_path(asset_as_pathbuf("firsttime/controls/background.png")));
		self.dpad_b_tex = None;
		self.a_b_tex = None;
		self.b_b_tex = None;
		self.start_b_tex = None;
		self.select_b_tex = None;
		self.lr_b_tex = None;
		
	}

	fn on_start(&mut self, _context: &mut GameContext) {
		
	}

}

impl Scene for FirstTimeControlsScene {
	
//	fn update(&mut self, _ctx: &mut Context, _context: &mut GameContext) {}
	
	fn render(&mut self, ctx: &mut Context, g: &mut GlGraphics, _tr: &mut TextRenderer) {
		draw_o(ctx, g, self.background_tex.as_ref(), 0, 0);
	}
	
	fn input(&mut self, context: &mut GameContext) { //[ButtonActions; 6]) {
		if context.keys[0] == 1 { //ButtonActions::PRESSED {
			
			self.scene_token = 8;
		}
	}
	
	fn dispose(&mut self) {}
	
	fn name(&self) -> &str {
		&"First Time - Controls"
	}
	
	fn next_scene(&self) -> usize {self.scene_token}
	
}

//firsttime narrative scene here
pub struct FirstTimeNarrativeScene {
	
	scene_token: usize,
	page: u8,
	background_tex: Option<Texture>,
	pikachu_tex: Option<Texture>, // change to pikachu sprite

}

impl FirstTimeNarrativeScene {
	
	pub fn new() -> FirstTimeNarrativeScene {
		FirstTimeNarrativeScene {
			scene_token: 0,
			page: 0,
			background_tex: None,
			pikachu_tex: None,
		}
	}
	
}

//
impl SceneLoad for FirstTimeNarrativeScene {

	fn load(&mut self, _context: &mut GameContext) {
		
		self.page = 0;
		self.background_tex = Some(texture_from_path(asset_as_pathbuf("firsttime/narrative/background.png")));
		self.pikachu_tex = None;
		
	}

	fn on_start(&mut self, _context: &mut GameContext) {
		
	}

}

impl Scene for FirstTimeNarrativeScene {
	
//	fn update(&mut self, _ctx: &mut Context, _context: &mut GameContext) {}
	
	fn render(&mut self, ctx: &mut Context, g: &mut GlGraphics, _tr: &mut TextRenderer) {
		draw_o(ctx, g, self.background_tex.as_ref(), 0, 0);
	}
	
	fn input(&mut self, context: &mut GameContext) { //[ButtonActions; 6]) {
		if context.keys[0] == 1 { //ButtonActions::PRESSED {
			
			self.scene_token = 6;
		}
	}
	
	fn dispose(&mut self) {}
	
	fn name(&self) -> &str {
		&"First Time - Narrative"
	}
	
	fn next_scene(&self) -> usize {self.scene_token}
	
}