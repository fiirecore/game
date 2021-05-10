use serde::{Deserialize, Serialize};

use deps::{
    hash::HashMap,
    vec::ArrayVec,
};

use util::Position;
use util::{
    Coordinate,
};

use crate::character::Character;
use crate::{TileId, MovementId};

use crate::map::{MapIdentifier, World, warp::WarpDestination};

use super::WorldChunk;

pub type Chunks = HashMap<MapIdentifier, WorldChunk>;

#[derive(Default, Deserialize, Serialize)]
pub struct WorldChunkMap {

    pub chunks: Chunks,

    #[serde(skip)]
    pub current: Option<MapIdentifier>,

}

impl WorldChunkMap {

    pub fn update_chunk(&mut self, id: MapIdentifier) -> Option<&WorldChunk> {
        if let Some(chunk) = self.chunks.get(&id) {
            self.current = Some(id);
            return Some(chunk);
        } else {
            return None;
        }
    }

    pub fn change_chunk(&mut self, id: MapIdentifier, position: &mut Position) {
        if let Some(chunk) = self.chunk() {
            let coords = chunk.coords;
            if let Some(chunk) = self.update_chunk(id) {
                {

                    position.coords = coords + position.coords - chunk.coords;
        
                    // player_pos.local.coords = player_pos.absolute() - chunk.coords;
                    // player_pos.global = chunk.coords;
                }            
            }  
        }      
    }

    pub fn chunk_at(&self, coords: Coordinate) -> Option<(&MapIdentifier, &WorldChunk)> {
        for chunk in &self.chunks {
            if chunk.1.in_bounds(coords) {
                return Some(chunk);
            }
        }
        None
    }

    pub fn chunk_id_at(&self, coords: Coordinate) -> Option<&MapIdentifier> {
        for (id, chunk) in &self.chunks {
            if chunk.in_bounds(coords - chunk.coords) {
                return Some(id);
            }
        }
        None
    }

    pub fn chunk(&self) -> Option<&WorldChunk> {
        self.current.as_ref().map(|id| self.chunks.get(id)).flatten()
    }

    pub fn chunk_mut(&mut self) -> Option<&mut WorldChunk> {
        self.current.map(move |id| self.chunks.get_mut(&id)).flatten()
    }

    pub fn connections(&self) -> ArrayVec<[(&MapIdentifier, &WorldChunk); 6]> {
        self.chunk().expect("Could not get current chunk").connections.iter().map(|connection| (connection, self.chunks.get(connection).expect("Could not get connected chunks"))).collect()
    }

    pub fn tiles(&self) -> Vec<crate::TileId> {
        let mut tiles = Vec::with_capacity(1000);
        for chunk in self.chunks.values() {
            for tile in &chunk.map.tiles {
                if !tiles.contains(tile) {
                    tiles.push(*tile);
                }
            }            
            for tile in &chunk.map.border {
                if !tiles.contains(tile) {
                    tiles.push(*tile);
                }
            }
        }
        return tiles;
    }

    pub fn walk_connections(&mut self, character: &mut Character, coords: Coordinate) -> (MovementId, bool) {
        let absolute = self.chunk().unwrap().coords + coords;
        let mut move_code = 1;
        let mut chunk = None;
        for (index, connection) in self.connections() {
            let connection_coords = absolute - connection.coords;
            if connection.in_bounds(connection_coords) {
                move_code = connection.walkable(connection_coords);
                chunk = Some(index.clone());
            }
        }
        if let Some(chunk) = chunk {
            if crate::map::manager::can_move(character.move_type, move_code) {
                self.change_chunk(chunk, &mut character.position);
                return (move_code, true);
            }
        }
        (move_code, false)
    }

}

impl World for WorldChunkMap {

    fn in_bounds(&self, coords: Coordinate) -> bool {
        self.chunk().map(|chunk| chunk.in_bounds(coords)).unwrap_or_default()
    }

    fn tile(&self, coords: Coordinate) -> Option<TileId> {
        match self.chunk() {
            Some(current) => {
                match current.tile(coords) {
                    Some(tile) => Some(tile),
                    None => {
                        for connection in &current.connections {
                            if let Some(connection) = self.chunks.get(connection) {
                                let tile = connection.tile(coords);
                                if tile.is_some() {
                                    return tile;
                                }
                            }
                        }
                        None
                    }
                }
            }
            None => {
                None
            }
        }
    }

    fn walkable(&self, coords: Coordinate) -> MovementId {
        self.chunk().map(|chunk| chunk.walkable(coords)).unwrap_or(1)
    }

    fn check_warp(&self, coords: Coordinate) -> Option<WarpDestination> {
        self.chunk().map(|chunk| chunk.check_warp(coords)).unwrap_or_default()
    }
    
}