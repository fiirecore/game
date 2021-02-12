use crate::util::Load;

use super::Scene;
use super::scenes::loading_scenes::*;

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

    copyright: LoadingCopyrightScene,
    gamefreak: LoadingGamefreakScene,
    pokemon: LoadingPokemonScene,
    current_scene: usize,
    pub finished: bool,

}

impl LoadingSceneManager {

    pub fn new() -> Self {
        Self {
            copyright: LoadingCopyrightScene::new(),
            gamefreak: LoadingGamefreakScene::new(),
            pokemon: LoadingPokemonScene::new(),
            current_scene: 0,
            finished: false,
        }
    }

    pub fn update(&mut self, delta: f32) {
        if !self.finished {
            match self.current_scene {
                0 => {
                    self.copyright.update(delta);
                    if self.copyright.next_scene() != 0 {
                        self.current_scene = self.copyright.next_scene();
                        self.gamefreak.on_start();
                    }
                },

                1 => {
                    self.gamefreak.update(delta);
                    if self.gamefreak.next_scene() != 0 {
                        self.current_scene = self.gamefreak.next_scene();
                        self.pokemon.on_start();
                    }
                },
                2 => {
                    self.pokemon.update(delta);
                    if self.pokemon.next_scene() != 0 {
                        self.current_scene = self.pokemon.next_scene();
                    }
                },
                _ => self.finish(),
            }
        }
    }

    pub fn render(&self) {
        if !self.finished {
            match self.current_scene {
                0 => self.copyright.render(),
                1 => self.gamefreak.render(),
                2 => self.pokemon.render(),
                _ => (),
            }
        }
    }

    fn finish(&mut self) {
        self.finished = true;
        macroquad::prelude::info!("Finished loading scene sequence.");
    }

}