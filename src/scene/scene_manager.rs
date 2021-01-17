use crate::util::traits::Loadable;
use crate::util::traits::PersistantData;
use log::info;
use opengl_graphics::GlGraphics;
use piston::UpdateArgs;
use piston_window::Context;
use crate::util::text_renderer::TextRenderer;

use crate::util::context::GameContext;

use crate::scene::scenes::loading_scenes::*;
use crate::scene::scenes::title_scene::TitleScene;
use crate::scene::scenes::firsttime_scenes::*;
use crate::scene::scenes::main_menu_scene::MainMenuScene;
use crate::scene::scenes::character_creation_scene::CharacterCreationScene;
use crate::scene::scenes::game_scene::GameScene;

use super::Scene;

pub struct SceneManager {

	pub current_scene_index: usize,
	pub scenes: Vec<Box<dyn Scene>>,
	loaded: Vec<bool>,

}

impl SceneManager {
	
	pub fn new() -> SceneManager {
		SceneManager {
			
			current_scene_index: 0,
			scenes: Vec::new(),
			loaded: Vec::new(),
		}
	}
	
	pub fn load_scene(&mut self, context: &mut GameContext, index: usize) {
		self.scenes[self.current_scene_index].dispose();
		self.current_scene_index = index;
		info!("Loading Scene: {}", self.scenes[self.current_scene_index].name());
		if !self.loaded[self.current_scene_index] {
			self.scenes[self.current_scene_index].load(context);
			self.loaded[self.current_scene_index] = true;
		}
		self.scenes[self.current_scene_index].on_start(context);
		self.check_scene(context);
	}
	
	pub fn load_scene_by_id(&mut self, context: &mut GameContext, id: usize) { //lmao put into map
		self.load_scene(context, id);
	}
	
	pub fn update(&mut self, _args: &UpdateArgs, context: &mut GameContext) {
		self.scenes[self.current_scene_index].update(context);
		self.check_scene(context);
	}
	
	pub fn render(&mut self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
		self.scenes[self.current_scene_index].render(ctx, g, tr);
	}
	
	pub fn input(&mut self, context: &mut GameContext){
		if context.fkeys[7] == 1 {
			info!("Attempting to reload configuration...");
			crate::CONFIGURATION.lock().unwrap().reload();
		}
		self.scenes[self.current_scene_index].input(context);		
	}

	fn check_scene(&mut self, context: &mut GameContext) {
		let id = self.scenes[self.current_scene_index].next_scene();
		if id != 0 {
			self.load_scene_by_id(context, id);
		}
	}
	
}

impl Loadable for SceneManager {

	fn load(&mut self) {
		
		self.scenes.push(Box::new(LoadingCopyrightScene::new()));
		self.scenes.push(Box::new(LoadingGamefreakScene::new()));
		self.scenes.push(Box::new(LoadingPokemonScene::new()));
		self.scenes.push(Box::new(TitleScene::new()));
		self.scenes.push(Box::new(MainMenuScene::new()));
		self.scenes.push(Box::new(GameScene::new()));
		self.scenes.push(Box::new(FirstTimeControlsScene::new()));
		self.scenes.push(Box::new(FirstTimeNarrativeScene::new()));
		self.scenes.push(Box::new(CharacterCreationScene::new()));

		for _index in 0..self.scenes.len() {
			self.loaded.push(false);
		}
		
	}

	fn on_start(&mut self, context: &mut GameContext) {
		self.load_scene_by_id(context, 0);
	}

	fn dispose(&mut self) {
		self.scenes[self.current_scene_index].dispose();
	}

}


