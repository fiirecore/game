use std::{collections::VecDeque, sync::Arc};

use battle::{
    moves::{ClientMove, ClientMoveAction},
    pokemon::{Indexed, PokemonIdentifier},
};
use pokengine::pokedex::{
    moves::Move,
    pokemon::{Experience, Level},
};

#[derive(Debug)]
pub struct MoveQueue<ID> {
    pub actions: VecDeque<Indexed<ID, BattleClientGuiAction<ID>>>,
    pub current: Option<Indexed<ID, BattleClientGuiCurrent<ID>>>,
}

#[derive(Debug, Clone)]
pub enum BattleClientGuiAction<ID> {
    Action(ClientMove<ID>),
    Catch,
    SetExp(Level, Experience, Vec<Arc<Move>>),
    LevelUp(Vec<Arc<Move>>),
    Replace(Option<usize>),
}

#[derive(Debug)]
pub enum BattleClientGuiCurrent<ID> {
    Move(Vec<Indexed<ID, ClientMoveAction>>),
    Switch(usize),
    UseItem(PokemonIdentifier<ID>),
    Catch,
    Replace(usize, bool),
    SetExp,
    LevelUp,
}
