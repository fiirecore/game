use crate::positions::{BoundingBox, Destination, Location};
use deps::{hash::HashMap, str::TinyStr16};
use serde::{Deserialize, Serialize};

pub type WarpId = TinyStr16;
pub type WarpMap = HashMap<WarpId, WarpEntry>;

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
