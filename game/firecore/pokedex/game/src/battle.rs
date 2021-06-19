use deps::{
    borrow::Identifiable, 
    hash::HashMap,
    tetra::graphics::Texture,
};

use pokedex::{
    Dex,
    moves::{Move, MoveId},
};

pub mod serialized;
pub mod script;

pub type BattleMoveRef = &'static BattleMove;

pub struct BattleMovedex;

static mut BATTLE_MOVE_DEX: Option<HashMap<MoveId, BattleMove>> = None;

impl Dex<'static> for BattleMovedex {
    type DexType = BattleMove;

    fn dex() -> &'static mut Option<HashMap<<<Self as Dex<'static>>::DexType as Identifiable<'static>>::Id, Self::DexType>> {
        unsafe { &mut BATTLE_MOVE_DEX }
    }
}

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
        BattleMovedex::try_get(id)
    }
}