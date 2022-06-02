use core::ops::Deref;
use std::collections::VecDeque;

use battle::{
    moves::{ClientMove, ClientMoveAction},
    pokemon::{Indexed, PokemonIdentifier},
};
use pokengine::pokedex::{
    moves::Move,
    pokemon::{Experience, Level},
};

#[derive(Debug)]
pub struct MoveQueue<ID, M: Deref<Target = Move>> {
    pub actions: VecDeque<Indexed<ID, BattleClientGuiAction<ID, M>>>,
    pub current: Option<Indexed<ID, BattleClientGuiCurrent<ID>>>,
}

#[derive(Debug, Clone)]
pub enum BattleClientGuiAction<ID, M: Deref<Target = Move>> {
    Action(ClientMove<ID>),
    Catch,
    SetExp(Level, Experience, Vec<M>),
    LevelUp(Vec<M>),
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
