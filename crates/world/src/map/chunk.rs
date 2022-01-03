use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use crate::positions::{Coordinate, CoordinateInt, Direction, Location};

use super::WorldMap;

pub type ChunkConnections = HashMap<Direction, Vec<Connection>>;
pub type ChunkOffset = CoordinateInt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldChunk {
    pub connections: ChunkConnections,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection(pub Location, pub ChunkOffset);

impl Connection {
    pub fn offset(direction: Direction, map: &WorldMap, offset: ChunkOffset) -> Coordinate {
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
