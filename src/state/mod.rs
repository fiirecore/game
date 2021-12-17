use firecore_battle_gui::pokedex::engine::Context;

use crate::{saves::{SavedPlayer, PlayerData}, command::CommandProcessor};

mod manager;
pub use manager::*;


pub mod game;
pub mod menu;

mod console;

#[derive(Debug, Clone)]
pub(crate) enum StateMessage {
    UseSave(SavedPlayer),
    UpdateSave(PlayerData),
    Save,
    Goto(MainStates),
    Seed(u8),
    Exit,
}

#[derive(Debug, Clone)]
pub enum MainStates {
    Menu,
    Game,
}

impl Default for MainStates {
    fn default() -> Self {
        Self::Menu
    }
}

pub trait MainState: CommandProcessor {

    fn draw(&self, ctx: &mut Context);

    fn end(&mut self, ctx: &mut Context);

}