use game::tetra::State;

mod manager;
pub use manager::*;

// pub mod first_scene;
pub mod title;
pub mod character;
pub mod main_menu;

pub trait MenuState: State {
	
	fn next(&mut self) -> &mut Option<MenuStateAction>;
	
}

#[derive(Clone, Copy)]
pub enum MenuStateAction {

    Goto(MenuStates),
    StartGame,

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