use deps::hash::HashMap;
use deps::str::TinyStr16;
use firecore_util::BoundingBox;
use firecore_util::Destination;
use serde::{Serialize, Deserialize};

use super::MapIdentifier;

pub type WarpId = TinyStr16;
pub type WarpMap = HashMap<WarpId, WarpEntry>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarpEntry {
    
    pub location: BoundingBox,

    pub destination: WarpDestination,

}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WarpDestination {

    pub map: Option<MapIdentifier>,
    pub index: MapIdentifier,

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