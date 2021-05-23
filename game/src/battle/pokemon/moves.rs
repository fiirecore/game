use crate::pokedex::{
    pokemon::{Level, Experience},
    moves::{
        MoveRef,
        target::MoveTargetInstance,
    },
    item::ItemRef,
};

use super::ActivePokemonIndex;

#[derive(Debug, Clone, Copy)]
pub enum BattleMove {

    Switch(usize),
    UseItem(ItemRef, ActivePokemonIndex),
    Move(usize, MoveTargetInstance),

}

#[derive(Debug)]
pub struct BattleActionInstance {
    pub pokemon: ActivePokemonIndex,
    pub action: BattleAction,
}

#[derive(Debug)]
pub enum BattleAction {
    Pokemon(BattleMove),
    Faint(Option<ActivePokemonIndex>), // user that made target faint
    Catch(ActivePokemonIndex),
    GainExp(Level, Experience),
    LevelUp(Level, Option<Vec<MoveRef>>),
    // Wait(f32),
}