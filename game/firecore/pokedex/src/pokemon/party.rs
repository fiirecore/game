use deps::vec::ArrayVec;

use super::instance::{
    PokemonInstance,
    BorrowedPokemon,
};

pub type PersistentParty = ArrayVec<[PokemonInstance; 6]>;
pub type MoveableParty = ArrayVec<[Option<BorrowedPokemon>; 6]>;