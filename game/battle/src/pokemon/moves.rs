use game::pokedex::item::ItemRef;
use game::pokedex::moves::MoveRef;
use game::pokedex::moves::target::MoveTargetInstance;

#[derive(Debug, Clone, Copy)]
pub enum BattleMove {

    Move(MoveRef, MoveTargetInstance),
    UseItem(ItemRef),
    Switch(usize),

}

#[derive(Debug, Clone, Copy)]
pub enum BattleAction {
    Pokemon(BattleMove),
    Faint(bool),
}