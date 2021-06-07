use deps::vec::ArrayVec;

use super::instance::{
    PokemonInstance,
    BorrowedPokemon,
};

pub type PokemonParty = ArrayVec<[PokemonInstance; 6]>;
pub type BorrowedParty = ArrayVec<[BorrowedPokemon; 6]>;