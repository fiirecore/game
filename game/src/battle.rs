use deps::tinystr::TinyStr8;
use pokedex::pokemon::party::PersistentParty;

use macroquad::prelude::Texture2D;

/***********************/

pub type BattleEntryRef<'a> = &'a mut Option<BattleEntry>;

pub struct BattleEntry {
    pub size: usize,
    pub party: PersistentParty,
    pub trainer: Option<BattleTrainerEntry>,
}

pub struct BattleTrainerEntry {
    pub prefix: String,
    pub name: String,
    pub transition: TinyStr8,
    pub texture: Texture2D,
    pub is_gym_leader: bool,
    pub victory_message: Vec<Vec<String>>,
    pub worth: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BattleTeam {
	Player,
	Opponent,
}

impl BattleTeam {

    pub const fn other(self) -> Self {
        match self {
            BattleTeam::Player => Self::Opponent,
            BattleTeam::Opponent => Self::Player,
        }
    }

}