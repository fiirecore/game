extern crate firecore_util as util;

use map::MapIdentifier;

pub mod map;
pub mod character;
pub mod script;

pub mod serialized;

pub type TileId = u16;
pub type MovementId = u8;
pub type MapSize = usize;

pub(crate) const fn default_true() -> bool {
    true
}

pub fn default_map_identifier() -> MapIdentifier {
    "".parse().unwrap()
}