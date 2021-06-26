use deps::vec::ArrayVec;

use super::instance::{
    PokemonInstance,
    BorrowedPokemon,
};

pub type Party<P> = ArrayVec<[P; 6]>;
pub type PokemonParty = Party<PokemonInstance>;
pub type BorrowedParty = Party<BorrowedPokemon>;