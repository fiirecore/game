extern crate firecore_dependencies as deps;
extern crate firecore_pokedex as pokedex;

pub mod moves;
pub mod texture;

pub mod serialize;

pub use pokedex::{
    Ref,
    Identifiable,
    BorrowableMut,
    pokemon,
    item,
};