use crate::engine::Context;

use crate::{command::CommandProcessor, saves::Player};

mod manager;
pub use manager::*;

pub mod game;
pub mod loading;
pub mod menu;

mod console;

#[derive(Debug, Clone)]
pub(crate) enum StateMessage {
    WriteSave,
    LoadSave,
    UpdateSave(Player),
    Goto(MainStates),
    Seed(u8),
    CommandError(&'static str),
    Exit,
}

#[derive(Debug, Clone)]
pub enum MainStates {
    Loading,
    Menu,
    Game,
}

impl Default for MainStates {
    fn default() -> Self {
        Self::Loading
    }
}

pub trait MainState: CommandProcessor {
    fn draw(&self, ctx: &mut Context);

    fn end(&mut self, ctx: &mut Context);
}
