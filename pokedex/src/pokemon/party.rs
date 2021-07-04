use deps::vec::ArrayVec;

use super::instance::PokemonInstance;

pub type Party<P> = ArrayVec<[P; 6]>;
pub type PokemonParty = Party<PokemonInstance>;
