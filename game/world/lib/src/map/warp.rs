use firecore_util::BoundingBox;
use firecore_util::Destination;
use serde::{Serialize, Deserialize};

use super::MapIdentifier;

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
    #[serde(default)] // remove
    pub transition: WarpTransition,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarpTransition {

    pub move_on_exit: bool,

    pub warp_on_tile: bool,

}

impl Default for WarpTransition {
    fn default() -> Self {
        Self {
            move_on_exit: false,
            warp_on_tile: true,
        }
    }
}