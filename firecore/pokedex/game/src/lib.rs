extern crate firecore_dependencies as deps;
extern crate firecore_pokedex as pokedex;

pub use pokedex::*;

pub mod battle2;
pub mod texture;

pub mod serialize;

mod init;

pub use init::*;