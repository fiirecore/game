use parking_lot::RwLock;

use crate::util::Load;

use super::Scene;
use super::scenes::loading_scenes::*;

lazy_static::lazy_static! {
    pub static ref LOADING_SCENE_FINISHED: RwLock<bool> = RwLock::new(false);
}

pub struct LoadingSceneManager {

    copyright: LoadingCopyrightScene,
    gamefreak: LoadingGamefreakScene,
    pokemon: LoadingPokemonScene,
    current_scene: usize,
    finished: bool,

}

impl LoadingSceneManager {

    pub fn new(sound: Option<kira::sound::handle::SoundHandle>) -> Self {
        Self {
            copyright: LoadingCopyrightScene::new(),
            gamefreak: LoadingGamefreakScene::new(sound),
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
                0 => self.copyright.render_notr(),
                1 => self.gamefreak.render_notr(),
                2 => self.pokemon.render_notr(),
                _ => (),
            }
        }
    }

    fn finish(&mut self) {
        self.finished = true;
        *LOADING_SCENE_FINISHED.write() = true;
        macroquad::prelude::info!("Finished loading scene sequence.");
    }

}