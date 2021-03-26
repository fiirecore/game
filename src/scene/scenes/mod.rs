pub mod first_scene;
pub mod title_scene;
pub mod game;
pub mod main_menu;

#[derive(Clone, Copy)]
pub enum SceneState {

    Continue,
    Scene(Scenes),
    // End,

}

#[derive(Clone, Copy)]
pub enum Scenes {

    // FirstLoadScene,
    TitleScene,
    MainMenuScene,

    GameScene,

}

impl Default for Scenes {
    fn default() -> Self {
        #[cfg(debug_assertions)] {
            Self::TitleScene
        }
        #[cfg(not(debug_assertions))] {
            Self::GameScene
        }
    }
}