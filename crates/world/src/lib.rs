pub extern crate firecore_audio as audio;
pub extern crate firecore_pokedex as pokedex;
pub extern crate firecore_text as text;

pub mod character;
pub mod map;
pub mod message;
pub mod positions;
pub mod random;
pub mod script;
pub mod serialized;
pub mod state;

pub const TILE_SIZE: f32 = 16.0;

const fn const_true() -> bool {
    true
}
