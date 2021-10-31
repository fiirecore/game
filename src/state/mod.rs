use engine::tetra::State;

use crate::GameContext;

mod manager;
pub use manager::*;

pub mod game;
pub mod menu;

// pub mod loading;

pub trait MainState<'d>: State<GameContext<'d>> {
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
        // #[cfg(not(debug_assertions))]
        // {
        //     Self::Menu
        // }
        // #[cfg(debug_assertions)]
        // {
            Self::Game
        // }
    }
}
