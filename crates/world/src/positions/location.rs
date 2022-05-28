use serde::{Deserialize, Serialize};

use super::Position;

pub type LocationId = tinystr::TinyStr16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Location {
    #[serde(default)]
    pub map: Option<LocationId>,
    pub index: LocationId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Spot {
    pub location: Location,
    pub position: Position,
}

impl From<LocationId> for Location {
    fn from(index: LocationId) -> Self {
        Self { map: None, index }
    }
}

impl core::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}/{}", self.map, self.index.as_str())
    }
}

impl Default for Location {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Location {
    pub const DEFAULT_INDEX: LocationId = unsafe {
        LocationId::from_bytes_unchecked([
            0x64, 0x65, 0x66, 0x61, 0x75, 0x6C, 0x74, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ])
    };
    pub const DEFAULT: Self = Self {
        map: None,
        index: Self::DEFAULT_INDEX,
    };
}
