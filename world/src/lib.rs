extern crate firecore_pokedex as pokedex;
extern crate firecore_text as text;

pub mod positions;
pub mod map;
pub mod character;
pub mod script;

pub mod serialized;

pub const TILE_SIZE: f32 = 16.0;

pub(crate) const fn default_true() -> bool {
    true
}

pub type TrainerId = tinystr::TinyStr16;