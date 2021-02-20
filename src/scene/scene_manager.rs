use game_scene::GameScene;

use crate::util::Quit;

use crate::scene::scenes::*;
use super::Scene;
use super::scenes::title_scene::TitleScene;
use super::scenes::main_menu_scene::MainMenuScene;

pub struct SceneManager {

	current_scene: Scenes,

	title_scene: TitleScene,
	main_menu_scene: MainMenuScene,
	game_scene: GameScene,

}

impl SceneManager {
	
	pub fn new() -> Self {
		Self {
			current_scene: Scenes::TitleScene,
			title_scene: TitleScene::new(),
			main_menu_scene: MainMenuScene::new(),
			game_scene: GameScene::new(),
		}
	}

	pub fn load_scene(&mut self, scene: Scenes) {
		self.get_mut().quit();
		self.current_scene = scene;
		// macroquad::prelude::debug!("Loading Scene: {}", self.scenes[self.current_scene_index].name());
		self.get_mut().on_start();
		self.check_scene();
	}
	
	pub fn update(&mut self, delta: f32) {
		self.get_mut().update(delta);
		self.check_scene();
	}
	
	pub fn render(&mut self) {
		self.get().render();
	}
	
	pub fn input(&mut self, delta: f32){
		self.get_mut().input(delta);		
	}

	fn check_scene(&mut self) {
		let id = self.get().next_scene();
		if let Some(id) = id {
			self.load_scene(id);
		}
	}

	pub fn on_start(&mut self) {
		self.load_scene(self.current_scene);
	}

	fn get(&self) -> &dyn Scene {
		match self.current_scene {
		    Scenes::TitleScene => &self.title_scene,
		    Scenes::MainMenuScene => &self.main_menu_scene,
		    Scenes::GameScene => &self.game_scene,
		}
	}

	fn get_mut(&mut self) -> &mut dyn Scene {
		match self.current_scene {
		    Scenes::TitleScene => &mut self.title_scene,
		    Scenes::MainMenuScene => &mut self.main_menu_scene,
		    Scenes::GameScene => &mut self.game_scene,
		}
	}
	
}

impl Quit for SceneManager {

	fn quit(&mut self) {
		self.get_mut().quit();
	}

}

