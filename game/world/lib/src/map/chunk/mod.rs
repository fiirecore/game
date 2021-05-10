use deps::vec::ArrayVec;
use serde::{Deserialize, Serialize};
use firecore_util::Coordinate;

use crate::MovementId;
use crate::TileId;

use crate::map::World;
use crate::map::warp::WarpDestination;

use super::MapIdentifier;
use super::WorldMap;

pub mod map;

pub type Connections = ArrayVec<[MapIdentifier; 6]>;

#[derive(Deserialize, Serialize)]
pub struct WorldChunk {

    // pub index: u16,

    pub coords: Coordinate,

    pub map: WorldMap,

    pub connections: Connections,

}

impl World for WorldChunk {

    fn in_bounds(&self, coords: Coordinate) -> bool {
        self.map.in_bounds(coords)
    }

    fn tile(&self, coords: Coordinate) -> Option<TileId> {
        self.map.tile(coords)
    }

    fn walkable(&self, coords: Coordinate) -> MovementId {
        if self.in_bounds(coords) {
            self.map.walkable(coords)
        } else {
            1
        }        
    }

    fn check_warp(&self, coords: Coordinate) -> Option<WarpDestination> {
        self.map.check_warp(coords)
    }

}