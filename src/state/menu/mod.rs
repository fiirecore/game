use crate::{engine::tetra::State, GameContext};

mod manager;
pub use manager::*;

// pub mod first_scene;
pub mod character;
pub mod main_menu;
pub mod title;

pub trait MenuState<'d>: State<GameContext<'d>> {
    fn next(&mut self) -> &mut Option<MenuStateAction>;
}

#[derive(Clone, Copy)]
pub enum MenuStateAction {
    Goto(MenuStates),
    StartGame,
    SeedAndGoto(u64, MenuStates),
}

#[derive(Clone, Copy)]
pub enum MenuStates {
    // FirstLoad,
    Title,
    MainMenu,

    CharacterCreation,
}

impl Default for MenuStates {
    fn default() -> Self {
        Self::Title
    }
}
