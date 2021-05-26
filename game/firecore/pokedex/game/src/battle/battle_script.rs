use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

// pub enum BattlePokemonActions {

// }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BattleActionScript {

    pub actions: VecDeque<BattleActionActions>,

    // #[serde(skip)]
    // pub texture: Option<Texture2D>,

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BattleActionActions {

    MoveAndReturn(f32),
    SpawnTexture,
    Wait(f32),
    // MoveTexture(f32, f32, f32),
    DespawnTexture,

}