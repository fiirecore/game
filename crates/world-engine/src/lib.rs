pub extern crate firecore_world as worldlib;
pub extern crate firecore_pokedex_engine as pokengine;

// pub(extern) use firecore_world as world;

pub use pokengine::engine;
pub use pokengine::pokedex;

pub mod map;
pub mod battle;
mod gui;

#[derive(Debug, Clone)]
pub enum WorldMetaAction {
    Battle(battle::BattleMessage),
}