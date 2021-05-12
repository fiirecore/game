use game::pokedex::item::ItemRef;
use game::pokedex::moves::target::MoveTargetInstance;
use game::pokedex::pokemon::types::Effective;

use super::ActivePokemonIndex;

#[derive(Debug, Clone, Copy)]
pub enum BattleMove {

    Switch(usize),
    UseItem(ItemRef),
    Move(usize, MoveTargetInstance),

}

#[derive(Debug, Clone, Copy)]
pub struct BattleActionInstance {
    pub pokemon: ActivePokemonIndex,
    pub action: BattleAction,
}

#[derive(Debug, Clone, Copy)]
pub enum BattleAction {
    Pokemon(BattleMove),
    Effective(Effective),
    Faint,
    // GainExp,
    // LevelUp,
    // Wait,
}