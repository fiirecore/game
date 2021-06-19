use serde::{Deserialize, Serialize};
use crate::pokedex::{
    item::ItemRef,
    moves::{target::MoveTargetInstance, MoveRef},
};

use super::{view::UnknownPokemon, ActivePokemonIndex, BattleClientMove, BattleMove};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActionInstance<T> {
    pub pokemon: ActivePokemonIndex,
    pub action: T,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BattleClientAction {
    Move(MoveRef, Vec<(MoveTargetInstance, Vec<BattleClientMove>)>),
    Switch(usize, Option<UnknownPokemon>),
    UseItem(ItemRef, MoveTargetInstance),
}

pub type BattleMoveInstance = ActionInstance<BattleMove>;
pub type BattleClientActionInstance = ActionInstance<BattleClientAction>;
