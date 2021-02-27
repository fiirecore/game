pub mod first_scene;
pub mod game_scene;
pub mod main_menu_scene;
pub mod title_scene;

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
        Self::TitleScene
    }
}