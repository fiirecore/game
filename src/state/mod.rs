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

#[derive(Debug, Clone)]
pub enum MainStates {
    Loading,
    Title,
    Menu,
    Game,
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
