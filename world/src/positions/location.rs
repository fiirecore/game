use serde::{Deserialize, Serialize};
use tinystr::TinyStr16;

pub type LocationId = TinyStr16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Location {
    #[serde(default)]
    pub map: Option<LocationId>,
    pub index: LocationId,
}

impl From<LocationId> for Location {
    fn from(index: LocationId) -> Self {
        Self { map: None, index }
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
