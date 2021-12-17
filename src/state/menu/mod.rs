mod manager;
pub use manager::*;

pub mod title;

pub mod main;
// pub mod first_scene;
// pub mod character;

#[derive(Clone)]
pub enum MenuActions {
    Goto(MenuStates),
    Seed(u8),
    StartGame(usize),
    ExitGame,
}

#[derive(Clone, Copy)]
pub enum MenuStates {
    // FirstLoad,
    Title,
    MainMenu,
}

impl Default for MenuStates {
    fn default() -> Self {
        Self::Title
    }
}
