use game::pokedex::item::ItemRef;
use game::pokedex::moves::MoveRef;
use game::pokedex::moves::target::MoveTargetInstance;
use game::pokedex::pokemon::Experience;
use game::pokedex::pokemon::Level;

use super::ActivePokemonIndex;

#[derive(Debug, Clone, Copy)]
pub enum BattleMove {

    Switch(usize),
    UseItem(ItemRef, Option<MoveTargetInstance>),
    Move(usize, MoveTargetInstance),

}

#[derive(Debug)]
pub struct BattleActionInstance {
    pub pokemon: ActivePokemonIndex,
    pub action: BattleAction,
}

#[derive(Debug)]
pub enum BattleAction {
    Pokemon(BattleMove),
    Faint(Option<ActivePokemonIndex>), // target
    Catch(ActivePokemonIndex),
    GainExp(Level, Experience),
    LevelUp(Level, Option<Vec<MoveRef>>),
    // Wait,
}