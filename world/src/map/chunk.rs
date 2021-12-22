use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::positions::{CoordinateInt, Direction, Location, Coordinate};

use super::WorldMap;

pub type ChunkConnections = HashMap<Direction, Connection>;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldChunk {
    pub connections: ChunkConnections,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection(pub Location, pub CoordinateInt);

impl Connection {

    pub fn offset(direction: Direction, map: &WorldMap, offset: i32) -> Coordinate {
        match direction {
            Direction::Down => Coordinate::new(offset, -1),
            Direction::Up => Coordinate::new(offset, map.height as _),
            Direction::Left => Coordinate::new(map.width as _, offset),
            Direction::Right => Coordinate::new(-1, offset),
        }
    }

}

impl From<ChunkConnections> for WorldChunk {
    fn from(connections: ChunkConnections) -> Self {
        Self { connections }
    }
}
