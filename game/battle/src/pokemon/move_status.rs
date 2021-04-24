use game::pokedex::item::ItemRef;
use game::pokedex::moves::MoveRef;

#[derive(Debug)]
pub struct BattleMoveStatus {

    pub action: BattleMoveType,
    pub queued: bool,

}


impl BattleMoveStatus {

    pub fn new(action: BattleMoveType) -> Self {
        Self {
            action,
            queued: true,
        }     
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BattleMoveType {

    Move(MoveRef),
    UseItem(ItemRef),
    Switch(usize),

}