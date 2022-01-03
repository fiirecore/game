use std::ops::Deref;

use pokedex::moves::Move;

use crate::{action::MoveQueue, transition::TransitionState};

#[derive(Debug)]
pub enum BattlePlayerState<ID, M: Deref<Target = Move>> {
    WaitToStart,
    Opening(TransitionState),
    Introduction(TransitionState),
    WaitToSelect,
    /// Current, Max
    Select(usize, usize),
    Moving(MoveQueue<ID, M>),
    PlayerEnd,
    GameEnd(Option<ID>),
    Closing(TransitionState),
}
