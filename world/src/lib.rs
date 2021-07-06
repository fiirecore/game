extern crate firecore_dependencies as deps;
extern crate firecore_pokedex as pokedex;
extern crate firecore_font as font;
extern crate firecore_audio as audio;

pub mod positions;
pub mod map;
pub mod character;
pub mod script;

pub mod serialized;

pub const TILE_SIZE: f32 = 16.0;

pub(crate) const fn default_true() -> bool {
    true
}