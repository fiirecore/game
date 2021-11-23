use crate::positions::{BoundingBox, Destination, Location};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tinystr::TinyStr16;

pub type WarpId = TinyStr16;
pub type Warps = HashMap<WarpId, WarpEntry>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WarpEntry {
    pub location: BoundingBox,
    pub destination: WarpDestination,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WarpDestination {
    pub location: Location,

    pub position: Destination,
    pub transition: WarpTransition,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WarpTransition {
    pub move_on_exit: bool,
    pub warp_on_tile: bool,

    #[serde(default = "crate::default_true")]
    pub change_music: bool,
}
