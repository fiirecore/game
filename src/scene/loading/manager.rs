use super::LoadingScene;
use super::LoadingScenes;
use super::LoadingState;
use super::copyright::CopyrightLoadingScene;
use super::gamefreak::GamefreakLoadingScene;

pub async fn load_coroutine() {
    macroquad::prelude::info!("Starting loading scene coroutine");
    let mut loading_scene_manager = LoadingSceneManager::new();
    while !loading_scene_manager.finished {
        loading_scene_manager.update(macroquad::prelude::get_frame_time());
        macroquad::prelude::clear_background(macroquad::prelude::BLACK);
        loading_scene_manager.render();
        macroquad::prelude::next_frame().await;
    }
}

pub struct LoadingSceneManager {
    
    current_scene: LoadingScenes,

    copyright: CopyrightLoadingScene,
    gamefreak: GamefreakLoadingScene,
    // pokemon: PokemonLoadingScene,
    pub finished: bool,

}

impl LoadingSceneManager {

    pub fn new() -> Self {
        Self {
            current_scene: LoadingScenes::default(),
            copyright: CopyrightLoadingScene::new(),
            gamefreak: GamefreakLoadingScene::new(),
            // pokemon: PokemonLoadingScene::new(),
            finished: false,
        }
    }

    pub fn update(&mut self, delta: f32) {
        if !self.finished {
            match self.get().state() {
                LoadingState::Continue => {
                    self.get_mut().update(delta);
                }
                LoadingState::Scene(scene) => {
                    self.current_scene = *scene;
                    self.get_mut().on_start();
                }
                LoadingState::End => {
                    self.finish();
                }
            }
        }
    }

    pub fn render(&self) {
        if !self.finished {
            self.get().render();
        }
    }

    fn finish(&mut self) {
        self.finished = true;
        macroquad::prelude::info!("Finished loading scene sequence.");
    }

    fn get(&self) -> &dyn LoadingScene {
        match self.current_scene {
            LoadingScenes::Copyright => &self.copyright,
            LoadingScenes::Gamefreak => &self.gamefreak,
        }
    }

    fn get_mut(&mut self) -> &mut dyn LoadingScene {
        match self.current_scene {
            LoadingScenes::Copyright => &mut self.copyright,
            LoadingScenes::Gamefreak => &mut self.gamefreak,
        }
    }

}