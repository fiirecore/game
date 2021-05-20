use serde::{Serialize, Deserialize};
use deps::vec::ArrayVec;

use crate::{
    // Identifiable,
    // pokemon::{
    //     PokemonId,
    //     Pokemon,
    //     Level,
    // },
    moves::{
        MoveRef,
        PP,
    }
};

pub type MoveInstanceSet = ArrayVec<[MoveInstance; 4]>;


// #[derive(Clone, Serialize, Deserialize)]
// pub enum MoveInstanceSet {
//     Init(ArrayVec<[MoveInstance; 4]>),
//     Uninit(PokemonId),
// }

// impl FromIterator<MoveRef> for MoveInstanceSet {
//     fn from_iter<T: IntoIterator<Item = MoveRef>>(iter: T) -> Self {
//         MoveInstanceSet::Init(iter.into_iter().enumerate().filter(|(index, _)| *index < 4).map(|(_, move_ref)| MoveInstance::new(move_ref)).collect())
//     }
// }

// impl MoveInstanceSet {
//     pub fn get_or_init(&mut self, index: usize, level: Level) -> &mut MoveInstance {
//         match self {
//             MoveInstanceSet::Init(moves) => &mut moves[index],
//             MoveInstanceSet::Uninit(id) => {
//                 self.init(level);
//                 match self {
//                     MoveInstanceSet::Init(moves) => &mut moves[index],
//                     MoveInstanceSet::Uninit(_) => unreachable!(),
//                 }
//             }
//         }
//     }
//     pub fn init(&mut self, level: Level) {
//         match self {
//             MoveInstanceSet::Init(_) => (),
//             MoveInstanceSet::Uninit(id) => match Pokemon::get(id) {
//                 crate::Ref::Init(pokemon) => {
//                     *self = pokemon.generate_moves(level);
//                 },
//                 crate::Ref::Uninit(id) => panic!("Move instance set and pokemon for pokemon with id {} was not initialized!", id),
//             }
//         }
//     }
//     // pub fn assume_init(&mut self, index: usize) -> &mut MoveInstance {
//     //     match self {
//     //         MoveInstanceSet::Init(moves) => &mut moves[index],
//     //         MoveInstanceSet::Uninit(id) => panic!("Move instance set for pokemon with id {} was not initialized!", id),
//     //     }
//     // }
//     pub fn push(&mut self, move_ref: MoveRef, level: Level) {
//         match self {
//             MoveInstanceSet::Init(moves) => moves.push(MoveInstance::new(move_ref)),
//             MoveInstanceSet::Uninit(_) => {
//                 self.init(level);
//                 self.push(move_ref, level);
//             }
//         }
//     }
// }

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MoveInstance {
    
    pub move_ref: MoveRef,
    pub pp: PP,
    
}

impl MoveInstance {

    pub fn new(move_ref: MoveRef) -> Self {
        Self {
            pp: move_ref.value().pp,
            move_ref,
        }
    }

    pub fn get(&self) -> Option<MoveRef> {
        (self.pp != 0).then(|| self.move_ref)
    }

    pub fn decrement(&mut self) -> Option<MoveRef> {
        (self.pp != 0).then(|| {
            self.pp -= 1;
            self.move_ref
        })
    }

}

