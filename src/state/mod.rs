use engine::tetra::{State, Context};

mod manager;
pub use manager::*;

pub mod menu;
pub mod game;

// pub mod loading;

#[deprecated]
pub trait NewState {
    fn new(ctx: &mut Context) -> Self where Self: Sized;
}

pub trait MainState: State {
    
    // #[deprecated(note = "fix so usable with other enums")]
    fn next(&mut self) -> Option<MainStates>;
    
}

// #[derive(Clone, Copy)]
pub enum MainStates {

	// Loading,
	Menu,
	Game,

}

impl Default for MainStates {
    fn default() -> Self {
        #[cfg(not(debug_assertions))] {
            Self::Menu
        }
        #[cfg(debug_assertions)] {
            Self::Game
        }
    }
}