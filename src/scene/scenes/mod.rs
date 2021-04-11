pub mod first_scene;
pub mod title;
pub mod game;
pub mod character;
pub mod main_menu;

// #[derive(Clone, Copy)]
// pub enum SceneState {

//     Continue,
//     Scene(Scenes),
//     // End,

// }

// #[derive(Clone, Copy)]
// pub enum Scenes {

//     // FirstLoadScene,
//     Title,
//     MainMenu,

//     CharacterCreation,

//     Game,

// }

// impl Default for Scenes {
//     fn default() -> Self {
//         #[cfg(not(debug_assertions))] {
//             Self::Title
//         }
//         #[cfg(debug_assertions)] {
//             Self::Game
//         }
//     }
// }