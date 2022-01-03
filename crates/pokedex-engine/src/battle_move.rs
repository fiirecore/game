use engine::tetra::graphics::Texture;

use pokedex::{moves::MoveId, Dex, Identifiable, IdentifiableRef, UNKNOWN_ID};

pub mod script;
pub mod serialized;

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

pub type BattleMoveRef<'d> = IdentifiableRef<'d, BattleMovedex>;

impl Identifiable for BattleMove {
    type Id = MoveId;

    const UNKNOWN: Self::Id = UNKNOWN_ID;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

pub type BattleMovedex = Dex<BattleMove>;
