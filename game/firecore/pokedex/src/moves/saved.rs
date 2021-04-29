use serde::{Deserialize, Serialize};
use deps::smallvec::SmallVec;
use super::{MoveId, PP};

pub type SavedMoveSet = SmallVec<[SavedMove; 4]>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SavedMove {
	pub id: MoveId,
	pub pp: Option<PP>,
}

pub fn to_instance(moves: &SavedMoveSet) -> super::instance::MoveInstanceSet {
    moves.iter().map(|saved_move| crate::movedex().get(&saved_move.id).map(|pokemon_move| super::instance::MoveInstance {
        pp: saved_move.pp.unwrap_or(pokemon_move.pp),
        pokemon_move: pokemon_move,
    })).flatten().collect()
}