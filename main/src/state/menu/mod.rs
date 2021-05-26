pub mod manager;

// pub mod first_scene;
pub mod title;
pub mod character;
pub mod main_menu;

pub trait MenuState {

	fn new() -> Self where Self: Sized;

	fn on_start(&mut self);
	
	fn update(&mut self, delta: f32);
	
	fn render(&self);
	
	fn quit(&mut self);
	
	fn action(&mut self) -> &mut Option<MenuStateAction>;
	
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