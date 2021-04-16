use firecore_util::Coordinate;
use serde::{Deserialize, Serialize};
use firecore_util::hash::HashMap;

use crate::MovementId;
use crate::TileId;

use super::MapIdentifier;
use super::World;
use super::WorldMap;
use super::warp::WarpDestination;

pub mod manager;

pub type Maps = HashMap<MapIdentifier, WorldMap>;

#[derive(Deserialize, Serialize)]
pub struct WorldMapSet {

    // pub name: String,
    pub maps: Maps,
    
    #[serde(skip)]
    pub current: Option<MapIdentifier>,

}

impl WorldMapSet {

    pub fn new(maps: Maps) -> Self {
        Self {
            // name,
            maps,
            current: None,
        }
    }

    pub fn map(&self) -> Option<&WorldMap> {
        self.current.as_ref().map(|id| self.maps.get(id)).flatten()
    }

    pub fn map_mut(&mut self) -> Option<&mut WorldMap> {
        self.current.map(move |id| self.maps.get_mut(&id)).flatten()
    }

}

impl World for WorldMapSet {

    fn in_bounds(&self, coords: Coordinate) -> bool {
        self.map().map(|map| map.in_bounds(coords)).unwrap_or(false)
    }

    fn tile(&self, coords: Coordinate) -> Option<TileId> {
        self.map().map(|map| map.tile(coords)).flatten()
    }

    fn walkable(&self, coords: Coordinate) -> MovementId {
        if self.in_bounds(coords) {
            if let Some(map) = self.map() {
                map.walkable(coords)
            } else {
                1
            }
        } else {
            1
        }
    }

    fn check_warp(&self, coords: Coordinate) -> Option<WarpDestination> {
        self.map().map(|map| map.check_warp(coords)).flatten()
    }

}