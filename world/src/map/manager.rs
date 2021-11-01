use std::ops::{Deref, DerefMut};

use crate::{
    character::Movement,
    map::{can_move, can_swim, can_walk, MovementId, TileId, WarpDestination, World, WorldMap},
    positions::{Coordinate, Direction, Location},
};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use self::data::WorldMapData;

pub mod data;

pub enum TryMoveResult {
    MapUpdate,
    TrySwim,
    StartWarpOnTile(TileId, Coordinate),
}

pub type Maps = HashMap<Location, WorldMap>;

#[derive(Deserialize, Serialize)]
pub struct WorldMapManager {
    pub maps: Maps,
    #[serde(skip)]
    pub data: WorldMapData,
}

// To - do: check surrounding maps
impl World for WorldMapManager {
    fn in_bounds(&self, coords: Coordinate) -> bool {
        self.get()
            .map(|map| map.in_bounds(coords))
            .unwrap_or_default()
    }

    fn tile(&self, coords: Coordinate) -> Option<TileId> {
        self.get().map(|map| map.tile(coords)).flatten()
    }

    fn movement(&self, coords: Coordinate) -> Option<MovementId> {
        self.get().map(|map| map.movement(coords)).flatten()
    }

    fn warp_at(&self, coords: Coordinate) -> Option<&WarpDestination> {
        self.get().map(|map| map.warp_at(coords)).flatten()
    }
}

impl WorldMapManager {
    pub fn get(&self) -> Option<&WorldMap> {
        self.location.as_ref().map(|id| self.maps.get(id)).flatten()
    }

    pub fn try_move(&mut self, direction: Direction) -> Option<TryMoveResult> {
        self.player.on_try_move(direction);

        let offset = direction.tile_offset();
        let coords = self.player.position.coords + offset;

        let movecode = self
            .movement(coords)
            .unwrap_or_else(|| self.walk_connections(coords).unwrap_or(1));

        let warp = match self.warp.is_none() {
            true => {
                if let Some(destination) = self.warp_at(coords) {
                    let warp_on_tile = destination.transition.warp_on_tile;
                    self.warp = Some(*destination);
                    if !warp_on_tile {
                        return Some(TryMoveResult::MapUpdate);
                    } else {
                        return Some(TryMoveResult::StartWarpOnTile(
                            self.get()
                                .map(|m| m.tile(coords))
                                .flatten()
                                .unwrap_or_default(),
                            coords,
                        ));
                    }
                } else {
                    false
                }
            }
            false => false,
        };

        fn one_way_tile(tile_id: TileId) -> bool {
            matches!(
                tile_id,
                135 | 176 | 177 | 143 | 151 | 184 | 185 | 192 | 193 | 217 | 1234
            )
        }

        let walk = self
            .tile(coords)
            .map(|tile_id| match direction {
                Direction::Up => false,
                Direction::Down => one_way_tile(tile_id),
                Direction::Left => tile_id == 133,
                Direction::Right => tile_id == 134,
            })
            .unwrap_or_default();

        let allow = warp || walk;

        let allow = if !allow {
            // checks if player is inside a solid tile or outside of map, lets them move if true
            // also checks if player is on a one way tile
            if self
                .tile(self.player.position.coords)
                .map(one_way_tile)
                .unwrap_or(false)
            {
                false
            } else {
                self.movement(self.player.position.coords)
                    .map(|code| !can_move(self.player.movement, code))
                    .unwrap_or(true)
            }
        } else {
            allow
        };

        if self.player.movement == Movement::Swimming && can_walk(movecode) {
            self.player.movement = Movement::Walking
        }

        if can_move(self.player.movement, movecode) || allow || self.player.noclip {
            self.player.pathing.queue.push(direction);
            // self.player.offset =
            //     direction.pixel_offset(self.player.speed() * 60.0 * delta);
        } else if can_swim(movecode) && self.player.movement != Movement::Swimming {
            return Some(TryMoveResult::TrySwim);
        }

        None
    }

    pub fn walk_connections(&mut self, coords: Coordinate) -> Option<MovementId> {
        if let Some(current) = self.get() {
            if let Some(chunk) = &current.chunk {
                let current_coords = chunk.coords;
                let absolute = current_coords + coords;
                for connection in chunk.connections.iter() {
                    if let Some(current) = self.maps.get(connection) {
                        if let Some(chunk) = &current.chunk {
                            if let Some(movement) = current.movement(absolute - chunk.coords) {
                                let c = current_coords - chunk.coords;
                                self.location = Some(*connection);
                                self.player.position.coords += c;
                                return Some(movement);
                            }
                        }
                    }
                }
            }
        }
        None
    }

    pub fn warp(&mut self, destination: WarpDestination) -> bool {
        match self.maps.contains_key(&destination.location) {
            true => {
                self.player.position.from_destination(destination.position);
                self.player.pathing.clear();
                self.location = Some(destination.location);
                true
            }
            false => todo!(),
        }
    }
}

impl From<Maps> for WorldMapManager {
    fn from(maps: Maps) -> Self {
        Self {
            maps,
            data: Default::default(),
        }
    }
}

impl Deref for WorldMapManager {
    type Target = WorldMapData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for WorldMapManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
