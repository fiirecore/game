use firecore_util::Coordinate;
use serde::{Serialize, Deserialize};
use firecore_util::hash::HashMap;

use crate::MovementId;
use crate::TileId;

use crate::map::MapIdentifier;
use crate::map::World;
use crate::map::warp::WarpDestination;

use super::WorldMapSet;

pub type Sets = HashMap<MapIdentifier, WorldMapSet>;

#[derive(Default, Serialize, Deserialize)]
pub struct WorldMapSetManager {

    pub map_sets: Sets,
    
    #[serde(skip)]
    pub current: Option<MapIdentifier>,

}

impl WorldMapSetManager {

    pub fn set_bank(&mut self, bank: MapIdentifier) {
        if self.map_sets.contains_key(&bank) {
            self.current = Some(bank);
        }
    }

    pub fn set_index(&mut self, index: MapIdentifier) {
        self.set_mut().map(|set| set.current = Some(index));
    }

    

    pub fn set(&self) -> Option<&WorldMapSet> {
        self.current.as_ref().map(|id| self.map_sets.get(id)).flatten()
    }

    pub fn set_mut(&mut self) -> Option<&mut WorldMapSet> {
        self.current.map(move |id| self.map_sets.get_mut(&id)).flatten()
    }

    pub fn tiles(&self) -> Vec<TileId> {
        let mut tiles = Vec::with_capacity(500);
        for map_set in self.map_sets.values() {
            for map in map_set.maps.values() {
                for tile in &map.tiles {
                    if !tiles.contains(tile) {
                        tiles.push(*tile);
                    }        
                }
                for tile in &map.border.tiles {
                    if !tiles.contains(tile) {
                        tiles.push(*tile);
                    }
                }
            }
        }
        return tiles;
    }

}

impl World for WorldMapSetManager {

    fn in_bounds(&self, coords: Coordinate) -> bool {
        self.set().map(|set| set.in_bounds(coords)).unwrap_or(false)
    }

    fn tile(&self, coords: Coordinate) -> Option<TileId> {
        self.set().map(|set| set.tile(coords)).unwrap_or_default()
    }

    fn walkable(&self, coords: Coordinate) -> MovementId {
        self.set().map(|set| set.walkable(coords)).unwrap_or(1)
    }

    fn check_warp(&self, coords: Coordinate) -> Option<WarpDestination> {
        self.set().map(|set| set.check_warp(coords)).unwrap_or_default()
    }

}