use super::{
    LoadingScene, LoadingState, LoadingScenes,
    copyright::CopyrightLoadingScene,
    gamefreak::GamefreakLoadingScene,
    pokemon::PokemonLoadingScene,
};

use game::macroquad::prelude::info;

pub struct LoadingSceneManager {
    
    current_scene: LoadingScenes,

    copyright: CopyrightLoadingScene,
    gamefreak: GamefreakLoadingScene,
    pokemon: PokemonLoadingScene,
    pub finished: bool,

}

impl LoadingSceneManager {

    pub fn new() -> Self {
        Self {
            current_scene: LoadingScenes::default(),
            copyright: CopyrightLoadingScene::new(),
            gamefreak: GamefreakLoadingScene::new(),
            pokemon: PokemonLoadingScene::new(),
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
                    self.current_scene = scene;
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
        info!("Finished loading scene sequence.");
    }

    fn get(&self) -> &dyn LoadingScene {
        match self.current_scene {
            LoadingScenes::Copyright => &self.copyright,
            LoadingScenes::Gamefreak => &self.gamefreak,
            LoadingScenes::Pokemon => &self.pokemon,
        }
    }

    fn get_mut(&mut self) -> &mut dyn LoadingScene {
        match self.current_scene {
            LoadingScenes::Copyright => &mut self.copyright,
            LoadingScenes::Gamefreak => &mut self.gamefreak,
            LoadingScenes::Pokemon => &mut self.pokemon,
        }
    }

}