use crate::positions::{BoundingBox, Destination, Location};
use serde::{Deserialize, Serialize};

// pub type WarpId = tinystr::TinyStr16;
pub type Warps = Vec<WarpEntry>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WarpEntry {
    pub area: BoundingBox,
    pub destination: WarpDestination,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WarpDestination {
    pub location: Location,
    /// Where the player will end up
    pub position: Destination,
}
