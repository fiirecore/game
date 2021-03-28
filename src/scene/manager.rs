use super::Scene;
use super::scenes::SceneState;
use super::scenes::Scenes;
use super::scenes::title::TitleScene;
use super::scenes::main_menu::MainMenuScene;
use super::scenes::game::GameScene;

pub struct SceneManager {

	current_scene: Scenes,

	title_scene: TitleScene,
	main_menu_scene: MainMenuScene,
	pub game_scene: GameScene,

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

	pub async fn load_all(&mut self) {
		self.title_scene.load().await;
		self.main_menu_scene.load().await;
		self.game_scene.load().await;
	}

	pub async fn poll(&mut self, delta: f32) {
		match self.get().state() {
		    SceneState::Continue => {
				self.get_mut().update(delta);
			}
		    SceneState::Scene(scene) => {
				self.get_mut().quit();
				self.current_scene = scene;
				self.get_mut().on_start().await;
			}
		}
	}
	
}

#[async_trait::async_trait(?Send)]
impl Scene for SceneManager {

	async fn load(&mut self) {

	}

    async fn on_start(&mut self) {
		#[cfg(debug_assertions)] {
			let mut saves = macroquad::prelude::collections::storage::get_mut::<firecore_data::player::list::PlayerSaves>().expect("Could not get player saves");
			if saves.saves.is_empty() {
				self.current_scene = Scenes::TitleScene;
			} else {
				saves.select(0);
			}			
		}
		let scene = self.get_mut();
        scene.load().await;
		scene.on_start().await;
    }

    fn input(&mut self, delta: f32) {
        self.get_mut().input(delta);
    }

    fn update(&mut self, _delta: f32) {
		macroquad::prelude::warn!("Use poll() for scene manager instead!");
	}

    fn render(&self) {
        self.get().render();
    }

	fn ui(&mut self) {
		self.get_mut().ui();
	}

    fn quit(&mut self) {
        self.get_mut().quit();
    }

    fn state(&self) -> SceneState {
        SceneState::Continue
    }
	
}
