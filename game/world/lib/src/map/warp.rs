use deps::hash::HashMap;
use deps::tinystr::TinyStr16;
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

#[serde(deny_unknown_fields)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarpDestination {

    pub map: Option<MapIdentifier>,
    pub index: MapIdentifier,

    pub position: Destination,
    pub transition: WarpTransition,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarpTransition {

    pub move_on_exit: bool,
    pub warp_on_tile: bool,

}