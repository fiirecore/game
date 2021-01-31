use crate::util::Quit;
use macroquad::prelude::info;
use crate::util::text_renderer::TextRenderer;
use crate::scene::scenes::*;
use super::Scene;

#[derive(Default)]
pub struct SceneManager {

	pub current_scene_index: usize,
	pub scenes: Vec<Box<dyn Scene>>,
	loaded: Vec<bool>,

}

impl SceneManager {
	
	pub fn load_scene(&mut self, index: usize) {
		self.scenes[self.current_scene_index].quit();
		self.current_scene_index = index;
		info!("Loading Scene: {}", self.scenes[self.current_scene_index].name());
		if !self.loaded[self.current_scene_index] {
			self.scenes[self.current_scene_index].load();
			self.loaded[self.current_scene_index] = true;
		}
		self.scenes[self.current_scene_index].on_start();
		self.check_scene();
	}
	
	pub fn load_scene_by_id(&mut self, id: usize) { //lmao put into map
		self.load_scene(id);
	}
	
	pub fn update(&mut self, delta: f32) {
		self.scenes[self.current_scene_index].update(delta);
		self.check_scene();
	}
	
	pub fn render(&mut self, tr: &TextRenderer) {
		self.scenes[self.current_scene_index].render(tr);
	}
	
	pub fn input(&mut self, delta: f32){
		// if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::F4) {
		// 	crate::io::data::configuration::Configuration::get().reload();
		// }
		self.scenes[self.current_scene_index].input(delta);		
	}

	fn check_scene(&mut self) {
		let id = self.scenes[self.current_scene_index].next_scene();
		if id != 0 {
			self.load_scene_by_id(id);
		}
	}

	pub async fn load_other_scenes(&mut self) {
		self.scenes.push(Box::new(title_scene::TitleScene::new()));
		self.scenes.push(Box::new(main_menu_scene::MainMenuScene::new()));
		self.scenes.push(Box::new(game_scene::GameScene::new()));
		// self.scenes.push(Box::new(firsttime_scenes::FirstTimeControlsScene::new().await));
		// self.scenes.push(Box::new(firsttime_scenes::FirstTimeNarrativeScene::new().await));
		// self.scenes.push(Box::new(character_creation_scene::CharacterCreationScene::new()));

		for _ in 0..self.scenes.len() {
			self.loaded.push(false);
		}
		
	}

	pub async fn load_loading_scenes(&mut self) {
		self.scenes.push(Box::new(loading_scenes::LoadingCopyrightScene::new()));
		self.scenes.push(Box::new(loading_scenes::LoadingGamefreakScene::new()));
		self.scenes.push(Box::new(loading_scenes::LoadingPokemonScene::new()));
	}

	pub fn on_start(&mut self) {
		self.load_scene_by_id(0);
	}
	
}

impl Quit for SceneManager {

	fn quit(&mut self) {
		self.scenes[self.current_scene_index].quit();
	}

}

