use serde::{Deserialize, Serialize};
use deps::vec::ArrayVec;
use util::{Coordinate, Location};

pub type Connections = ArrayVec<[Location; 6]>;

#[derive(Serialize, Deserialize)]
pub struct WorldChunk {

    pub connections: Connections,

    pub coords: Coordinate,

}