use game_scene::GameScene;

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
			current_scene: Scenes::default(),
			title_scene: TitleScene::new(),
			main_menu_scene: MainMenuScene::new(),
			game_scene: GameScene::new(),
		}
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

impl Scene for SceneManager {
    fn on_start(&mut self) {
        self.get_mut().on_start();
    }

    fn update(&mut self, delta: f32) {
        match self.get().state() {
		    SceneState::Continue => {
				self.get_mut().update(delta);
			}
		    SceneState::Scene(scene) => {
				self.get_mut().quit();
				self.current_scene = scene;
				self.get_mut().on_start();
			}
		}
    }

    fn render(&self) {
        self.get().render();
    }

    fn input(&mut self, delta: f32) {
        self.get_mut().input(delta);
    }

    fn quit(&mut self) {
        self.get_mut().quit();
    }

    fn state(&self) -> SceneState {
        SceneState::Continue
    }
	
}
