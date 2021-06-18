use deps::{
    borrow::Identifiable, 
    tetra::graphics::Texture,
};

use pokedex::moves::{Move, MoveId};

pub mod serialized;

pub mod dex;

pub mod script;

pub type BattleMoveRef = &'static BattleMove;

#[derive(Debug, Clone)]
pub struct BattleMove {

    pub id: MoveId,

    pub texture: Option<Texture>,

    pub script: script::BattleActionScript,

}

impl BattleMove {

    pub fn script(&self) -> script::BattleActionScriptInstance {
        script::BattleActionScriptInstance {
            script: self.script.clone(),
            texture: self.texture.clone(),
        }
    }

}

impl<'a> Identifiable<'a> for BattleMove {

    type Id = MoveId;

    const UNKNOWN: Self::Id = Move::UNKNOWN;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn try_get(id: &Self::Id) -> Option<&'a Self> where Self: Sized {
        unsafe { dex::BATTLE_MOVE_DEX.as_ref().map(|dex| dex.get(id)).flatten() }
    }
}