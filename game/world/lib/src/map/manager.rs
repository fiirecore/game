use std::u8;

use serde::{Deserialize, Serialize};

use firecore_util::Direction;

use crate::MovementId;
use crate::character::MoveType;
// use crate::character::Character;
use crate::character::player::PlayerCharacter;

use super::MapIdentifier;
use super::World;
use super::chunk::map::WorldChunkMap;
use super::set::manager::WorldMapSetManager;
use super::warp::WarpDestination;

pub enum TryMoveResult {
    MapUpdate,
    TrySwim,
}

#[derive(Default, Deserialize, Serialize)]
pub struct WorldMapManager {

    pub chunk_map: WorldChunkMap,
    pub map_set_manager: WorldMapSetManager,

    #[serde(skip)]
    pub chunk_active: bool,

    #[serde(skip)]
    pub player: PlayerCharacter,

    #[serde(skip)]
    pub warp: Option<WarpDestination>,

}

impl WorldMapManager {

    pub fn try_move(&mut self, direction: Direction, delta: f32) -> Option<TryMoveResult> { // return chunk update

        let mut update = false;

        self.player.character.on_try_move(direction);

        let offset = direction.tile_offset();
        let coords = self.player.character.position.coords + offset;

        let in_bounds = if self.chunk_active {
            self.chunk_map.in_bounds(coords)
        } else {
            self.map_set_manager.in_bounds(coords)
        };

        let move_code = if self.chunk_active {
            if in_bounds {
                self.chunk_map.walkable(coords)
            } else {
               let (code, do_update) = self.chunk_map.walk_connections(&mut self.player.character, coords);
               update = do_update;
               code
            }
        } else {
            if in_bounds {
                self.map_set_manager.walkable(coords)
            } else {
                1
            }
        };

        let allow = if self.chunk_active && self.warp.is_none() {
            if let Some(destination) = self.chunk_map.check_warp(coords) {
                if !destination.transition.warp_on_tile {
                    self.warp = Some(destination);
                    return Some(TryMoveResult::MapUpdate);
                } else {
                    true
                }
            } else {
                false
            }
        } else {
            if let Some(destination) = self.map_set_manager.check_warp(coords) {
                if !destination.transition.warp_on_tile {
                    self.warp = Some(destination);
                    return Some(TryMoveResult::MapUpdate);
                } else {
                    true
                }
            } else {
                false
            }
        } || if let Some(tile_id) = if self.chunk_active {
            self.chunk_map.tile(coords)
        } else {
            self.map_set_manager.tile(coords)
        } {
            match direction {
                Direction::Up => false,
                Direction::Down => match tile_id  {
                    135 | 176 | 177 | 143 | 151 | 184 | 185 | 192 | 193 | 217 | 1234 => true,
                    _ => false,
                },
                Direction::Left => tile_id == 133,
                Direction::Right => tile_id == 134,
            }
        } else {
            false
        };

        if self.player.character.move_type == MoveType::Swimming && can_walk(move_code) {
            self.player.character.move_type = MoveType::Walking
        }

        if can_move(self.player.character.move_type, move_code) || allow || self.player.character.noclip {
            let mult = self.player.character.speed() * 60.0 * delta;
            self.player.character.position.offset = direction.pixel_offset().scale(mult);
            self.player.character.moving = true;
        } else if can_swim(move_code) && self.player.character.move_type != MoveType::Swimming {
            return Some(TryMoveResult::TrySwim);
        }

        if update {
            Some(TryMoveResult::MapUpdate)
        } else {
            None
        }
    }

    pub fn update_chunk(&mut self, index: MapIdentifier) {
        self.chunk_map.update_chunk(index);
    }

    pub fn update_map_set(&mut self, bank: MapIdentifier, index: MapIdentifier) {
        self.map_set_manager.set_bank(bank);
        self.map_set_manager.set_index(index);
    }

    pub fn warp(&mut self, destination: WarpDestination) {
        if destination.map.is_none() {
            self.warp_to_chunk_map(destination);
        } else {
            self.warp_to_map_set(destination);
        }
    }

    pub fn warp_to_chunk_map(&mut self, destination: WarpDestination) {
        if !self.chunk_active {
            self.chunk_active = true;
        }
        if self.chunk_map.update_chunk(destination.index).is_some() {
            self.player.character.position.from_destination(destination.position);
        }
    }

    pub fn warp_to_map_set(&mut self, destination: WarpDestination) {
        if self.chunk_active {
            self.chunk_active = false;
        }
        self.update_map_set(destination.map.unwrap(), destination.index);
        self.player.character.position.from_destination(destination.position);
    }

}

pub fn can_move(move_type: MoveType, code: MovementId) -> bool {
    match move_type {
        MoveType::Swimming => can_swim(code),
        _ => can_walk(code),
    }
}

pub fn can_walk(code: MovementId) -> bool {
    code == 0xC
    // match move_code {
    //     0x0C | 0x00 | 0x04 => true,
    //     _ => false,
    // }
}

pub fn can_swim(code: MovementId) -> bool {
    code == 0x4
}