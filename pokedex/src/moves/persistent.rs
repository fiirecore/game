use serde::{Deserialize, Serialize};
use super::Move;
use super::usage::MoveUseType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistentMove {

    pub length: Option<(u8, u8)>, // min,max

    pub action: MoveUseType,

    pub same_move: bool,
    
    // pub secondary: Option<MoveActionType>,

}

#[derive(Debug, Clone)]
pub struct PersistentMoveInstance {
    pub pokemon_move: &'static Move,
    pub actions: MoveUseType,
    pub remaining: Option<u8>,
    pub same_move: bool, // what does this bool mean?
}