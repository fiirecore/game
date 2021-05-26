use serde::{Deserialize, Serialize};

use pokedex::moves::MoveId;

pub mod dex;

// pub mod battle_script;

pub type BattleMoveRef = &'static BattleMove;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleMove {
 
    // pub plugin: Option<String>,

}

impl BattleMove {

    pub fn try_get(id: &MoveId) -> Option<BattleMoveRef> {
        unsafe { dex::BATTLE_MOVE_DEX.as_ref().map(|dex| dex.get(id)).flatten() }
    }

}