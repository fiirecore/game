use crate::engine::State;

use crate::GameContext;

mod manager;
pub use manager::*;

pub mod game;
pub mod menu;

// pub mod loading;

pub trait MainState: State<GameContext> {
    fn action(&mut self) -> Option<Action>;
}

pub enum Action {
    Goto(MainStates),
    Seed(u64),
}

// #[derive(Clone, Copy)]
pub enum MainStates {
    // Loading,
    Menu,
    Game,
}

impl Default for MainStates {
    fn default() -> Self {
        #[cfg(not(any(debug_assertions, target_arch = "wasm32")))]
        {
            Self::Menu
        }
        #[cfg(any(debug_assertions, target_arch = "wasm32"))]
        {
            Self::Game
        }
    }
}
