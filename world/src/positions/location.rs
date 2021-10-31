use tinystr::TinyStr16;
use serde::{Deserialize, Serialize};

pub type LocationId = TinyStr16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Location {
    pub map: Option<LocationId>,
    pub index: LocationId,
}

impl Location {
    pub const fn new(map: Option<LocationId>, index: LocationId) -> Self {
        Self { map, index }
    }
}

impl core::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}/{}", self.map, self.index)
    }
}

impl Default for Location {
    fn default() -> Self {
        Self {
            map: None,
            index: "default".parse().unwrap(),
        }
    }
}
