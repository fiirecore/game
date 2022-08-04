mod manager;
pub use manager::*;
use worldcli::engine::graphics::Draw;

pub mod game;
pub mod loading;
pub mod menu;

mod console;

#[derive(Debug, Clone)]
pub(crate) enum StateMessage {
    ResetSave,
    SaveToDisk,
    Goto(MainStates),
    Seed(u8),
    Exit,
}

#[derive(Debug, Clone, Copy)]
pub enum MainStates {
    // Boot,
    Loading,
    Title,
    Menu,
    Game,
    Error(&'static str),
}

impl Default for MainStates {
    fn default() -> Self {
        Self::Loading
    }
}

pub trait MainState {
    fn draw(&self, draw: &mut Draw);

    fn end(&mut self, ctx: &mut Draw);
}

#[derive(Debug, Default)]
struct StateManager<S> {
    current: S,
    next: Option<S>,
}

impl<S> StateManager<S> {
    pub fn queue(&mut self, state: S) {
        self.next = Some(state);
    }

    /// Next is taken and given back to state manager in this function
    pub fn update(&mut self, next: S) {
        self.current = next;
        self.next = None;
    }

}