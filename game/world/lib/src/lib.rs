extern crate firecore_dependencies as deps;
extern crate firecore_util as util;

pub mod map;
pub mod character;
pub mod script;

pub mod serialized;

pub type PaletteId = u8;
pub type TileId = u16;
pub type MovementId = u8;
pub type MapSize = usize;

pub(crate) const fn default_true() -> bool {
    true
}