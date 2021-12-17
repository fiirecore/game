use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::positions::{CoordinateInt, Direction, Location, Coordinate};

use super::WorldMap;

pub type ChunkConnections = HashMap<Direction, Connection>;

#[derive(Serialize, Deserialize)]
pub struct WorldChunk {
    pub connections: ChunkConnections,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection(pub Location, pub CoordinateInt);

impl Connection {

    pub fn offset(direction: Direction, map: &WorldMap, offset: i32) -> Coordinate {
        match direction {
            Direction::Down => Coordinate::new(offset, 0),
            Direction::Up => Coordinate::new(offset, (map.height - 1) as _),
            Direction::Left => Coordinate::new((map.width - 1) as _, offset),
            Direction::Right => Coordinate::new(0, offset),
        }
    }

}

impl From<ChunkConnections> for WorldChunk {
    fn from(connections: ChunkConnections) -> Self {
        Self { connections }
    }
}
