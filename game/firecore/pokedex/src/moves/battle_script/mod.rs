use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

// pub enum BattlePokemonActions {

// }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BattleActionScript {

    pub actions: VecDeque<BattleActionActions>,

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BattleActionActions {

    MoveAndReturn(f32),

}