use serde::{Deserialize, Serialize};
use super::MoveRef;
use super::script::MoveActionType;

pub type MoveActionRef = &'static MoveActionType;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PersistentMove {

    pub length: Option<(u8, u8)>,

    pub action: MoveActionType,

    pub on_move: bool,
    
    // pub secondary: Option<MoveActionType>,

}

#[derive(Debug, Clone)]
pub struct PersistentMoveInstance {
    pub pokemon_move: MoveRef,
    pub actions: MoveActionRef,
    pub remaining: Option<u8>,
    pub should_do: bool,
}