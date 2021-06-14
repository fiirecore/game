extern crate firecore_dependencies as deps;
extern crate firecore_util as util;
extern crate firecore_pokedex as pokedex;
extern crate firecore_font as font;
extern crate firecore_audio_lib as audio;

pub mod map;
pub mod character;
pub mod script;

pub mod serialized;

pub(crate) const fn default_true() -> bool {
    true
}