use std::collections::VecDeque;

use engine::tetra::{graphics::Texture, math::Vec2};
use serde::{Deserialize, Serialize};

// pub enum BattlePokemonActions {

// }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BattleActionScript {
    pub actions: VecDeque<BattleAction>,

    #[serde(skip)]
    pub current: Option<BattleActionInstance>,

    #[serde(skip)]
    pub texture: Option<Vec2>,
    // #[serde(skip)]
    // pub texture: Option<Texture2D>,
}

#[derive(Debug, Clone)]
pub struct BattleActionScriptInstance {
    pub script: BattleActionScript,
    pub texture: Option<Texture>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BattleAction {
    MoveAndReturnPokemon(f32),
    MoveTexture(f32, f32, f32),
    SpawnTexture(f32, f32),
    Wait(f32),
    // MoveTexture(f32, f32, f32),
    DespawnTexture,
}

#[derive(Debug, Clone)]
pub enum BattleActionInstance {
    MoveAndReturn(f32, f32, bool),
    MoveTexture(Vec2, f32),
    SpawnTexture(f32, f32),
    Wait(f32),
    DespawnTexture,
}
