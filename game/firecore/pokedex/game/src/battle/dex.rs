use deps::hash::HashMap;
use pokedex::moves::MoveId;
use super::BattleMove;

pub type BattleMoveDex = HashMap<MoveId, BattleMove>;

pub(crate) static mut BATTLE_MOVE_DEX: Option<BattleMoveDex> = None;

pub fn set(dex: BattleMoveDex) {
    unsafe { BATTLE_MOVE_DEX = Some(dex) }
}