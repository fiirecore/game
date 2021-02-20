pub mod first_scene;
// pub mod character_creation_scene;
// pub mod firsttime_scenes;
pub mod game_scene;
pub mod loading_scenes;
pub mod main_menu_scene;
pub mod title_scene;

pub enum LoadingScenes {

    LoadingCopyrightScene,
    LoadingGamefreakScene,
    LoadingPokemonScene,

}

#[derive(Clone, Copy)]
pub enum Scenes {

    // FirstLoadScene,
    TitleScene,
    MainMenuScene,

    GameScene,

}