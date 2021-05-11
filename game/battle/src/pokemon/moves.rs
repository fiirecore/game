use game::pokedex::item::ItemRef;
use game::pokedex::moves::MoveRef;
use game::pokedex::moves::target::MoveTargetInstance;
use game::pokedex::pokemon::types::effective::Effective;

use super::ActivePokemonIndex;

#[derive(Debug, Clone, Copy)]
pub enum BattleMove {

    Switch(usize),
    UseItem(ItemRef),
    Move(MoveRef, MoveTargetInstance),

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
    LevelUp,
    // Wait,
}