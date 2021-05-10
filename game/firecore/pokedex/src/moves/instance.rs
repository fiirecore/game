use deps::vec::ArrayVec;
use super::PP;

use super::MoveRef;

pub type MoveInstanceSet = ArrayVec<[MoveInstance; 4]>;


#[derive(Debug, Clone, Copy)]
pub struct MoveInstance {
    
    pub pokemon_move: MoveRef,
    pub pp: PP,
    
}

impl MoveInstance {

    pub fn new(pokemon_move: MoveRef) -> Self {
        Self {
            pp: pokemon_move.pp,
            pokemon_move,
        }
    }

    pub fn use_move(&mut self) -> Option<MoveRef> {
        if self.pp == 0 {
            None
        } else {
            self.pp -= 1;
            Some(self.pokemon_move)
        }
        
    }

}

pub fn to_saved(moves: MoveInstanceSet) -> super::saved::SavedMoveSet {
    moves.into_iter().map(|instance| super::saved::SavedMove {
        id: instance.pokemon_move.id,
        pp: Some(instance.pp),
    }).collect()
}

