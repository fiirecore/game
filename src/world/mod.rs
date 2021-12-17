

pub mod manager;

pub mod map;

mod battle;
mod gui;
mod npc;
mod screen;

use firecore_battle::pokedex::{pokemon::owned::SavedPokemon, item::SavedItemStack};
use firecore_battle_gui::pokedex::engine::{gui::MessagePage, graphics::Color};
pub use screen::RenderCoords;
use worldlib::script::ScriptId;

use crate::game::battle_glue::BattleEntry;

#[derive(Clone)]
pub enum WorldActions {
    Battle(BattleEntry),
    GivePokemon(SavedPokemon),
    GiveItem(SavedItemStack),
    HealPokemon(Option<usize>),
    Message(Vec<MessagePage>, Option<Color>, bool),
    #[deprecated]
    EndScript(ScriptId),
}