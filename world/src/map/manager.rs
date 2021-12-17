use crate::{
    character::{player::PlayerCharacter, Movement},
    map::{movement::{can_move, can_swim, can_walk}, MovementId, TileId, WarpDestination, WorldMap},
    positions::{Coordinate, Direction, Location},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{chunk::Connection, movement::MovementResult};

pub mod state;

pub enum TryMoveResult {
    MapUpdate,
    TrySwim,
    StartWarpOnTile(TileId, Coordinate),
}

pub type Maps = HashMap<Location, WorldMap>;

#[derive(Default, Deserialize, Serialize)]
pub struct WorldMapManager {
    pub maps: Maps,
}

impl WorldMapManager {
    pub fn get(&self, location: &Location) -> Option<&WorldMap> {
        self.maps.get(location)
    }

    pub fn try_move(
        &mut self,
        player: &mut PlayerCharacter,
        direction: Direction,
    ) -> Option<TryMoveResult> {
        player.on_try_move(direction);

        let offset = direction.tile_offset();
        let coords = player.position.coords + offset;

        let movecode = self
            .get(&player.location)
            .map(|map| match map.movement(coords) {
                MovementResult::Option(code) => code,
                MovementResult::Chunk(direction, offset, connection) => self
                    .connection_movement(direction, offset, connection)
                    .map(|(coords, code)| {
                        player.character.position.coords = coords;
                        player.location = connection.0;
                        code
                    }),
            })
            .flatten()
            .unwrap_or(1);
        // .unwrap_or_else(|| self.walk_connections(coords).unwrap_or(1));

        if let Some(map) = self.get(&player.location) {

            let warp = match player.world.warp.is_none() {
                true => {
                    if let Some(destination) = map.warp_at(coords) {
                        let warp_on_tile = destination.transition.warp_on_tile;
                        player.world.warp = Some(*destination);
                        if !warp_on_tile {
                            return Some(TryMoveResult::MapUpdate);
                        } else {
                            return Some(TryMoveResult::StartWarpOnTile(
                                map.tile(coords).unwrap_or_default(),
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

            

            let walk = map
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
                if map
                    .tile(player.position.coords)
                    .map(one_way_tile)
                    .unwrap_or(false)
                {
                    false
                } else {
                    !can_move(player.movement, movecode)
                }
            } else {
                allow
            };

            if player.movement == Movement::Swimming && can_walk(movecode) {
                player.movement = Movement::Walking
            }

            if can_move(player.movement, movecode) || allow || player.noclip {
                player.pathing.queue.push(direction);
                // self.player.offset =
                //     direction.pixel_offset(self.player.speed() * 60.0 * delta);
            } else if can_swim(movecode) && player.movement != Movement::Swimming {
                return Some(TryMoveResult::TrySwim);
            }
        }

        None
    }

    pub fn connection_movement(
        &self,
        direction: Direction,
        offset: i32,
        connection: &Connection,
    ) -> Option<(Coordinate, MovementId)> {
        self.get(&connection.0)
            .map(|map| {
                let o = offset - connection.1;
                let coords = Connection::offset(direction, map, o);
                map.local_movement(coords).map(|code| (coords, code))
            })
            .flatten()
    }

    // pub fn walk_connections(&mut self, coords: Coordinate) -> Option<MovementId> {
    //     if let Some(current) = self.get() {
    //         if let Some(chunk) = &current.chunk {
    //             let current_coords = chunk.coords;
    //             let absolute = current_coords + coords;
    //             for connection in chunk.connections.iter() {
    //                 if let Some(current) = self.maps.get(connection) {
    //                     if let Some(chunk) = &current.chunk {
    //                         if let Some(movement) = current.movement(absolute - chunk.coords) {
    //                             let c = current_coords - chunk.coords;
    //                             self.location = Some(*connection);
    //                             self.player.position.coords += c;
    //                             return Some(movement);
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     None
    // }

    pub fn warp(&mut self, player: &mut PlayerCharacter, destination: WarpDestination) -> bool {
        match self.maps.contains_key(&destination.location) {
            true => {
                player.position.from_destination(destination.position);
                player.pathing.clear();
                player.location = destination.location;
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
        }
    }
}
