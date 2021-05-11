use firecore_util::battle::BattleType;

use pokedex::pokemon::instance::PokemonInstanceParty;

use macroquad::prelude::Texture2D;

/***********************/

pub type BattleEntryRef<'a> = &'a mut Option<BattleEntry>;

pub struct BattleEntry {

    pub size: usize,
    pub party: PokemonInstanceParty,
    pub trainer: Option<BattleTrainerEntry>,

}

pub struct BattleTrainerEntry {


    pub name: String,
    pub npc_type: String,
    pub battle_type: BattleType,
    pub texture: Texture2D,
    pub victory_message: Vec<Vec<String>>,
    pub worth: u16,

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BattleTeam {

	Player,
	Opponent,

}

impl BattleTeam {

    pub fn other(self) -> Self {
        match self {
            BattleTeam::Player => Self::Opponent,
            BattleTeam::Opponent => Self::Player,
        }
    }

}

impl BattleEntry {

    pub fn get_type(&self) -> BattleType {
        self.trainer.as_ref().map(|data| data.battle_type).unwrap_or(BattleType::Wild)
    }

}