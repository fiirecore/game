use serde::{Deserialize, Serialize};
use deps::vec::ArrayVec;
use super::{MoveId, PP};

pub type SavedMoveSet = ArrayVec<[SavedMove; 4]>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SavedMove {
	pub id: MoveId,
	pub pp: Option<PP>,
}

pub fn to_instance(moves: &SavedMoveSet) -> super::instance::MoveInstanceSet {
    moves.iter().flat_map(|saved_move| super::movedex().get(&saved_move.id).map(|pokemon_move| super::instance::MoveInstance {
        pp: saved_move.pp.unwrap_or(pokemon_move.pp),
        pokemon_move: pokemon_move,
    })).collect()
}