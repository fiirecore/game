use serde::{Deserialize, Serialize};

#[cfg(feature = "pathfind")]
pub mod astar;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum MovementType {

    Still,
    LookAround,
    WalkUpAndDown(isize),

}

impl Default for MovementType {
    fn default() -> Self {
        Self::Still
    }
}

#[cfg(feature = "pathfind")]
use {
    firecore_util::{Direction, Position, Destination},
    crate::map::World
};

#[cfg(feature = "pathfind")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DestinationPath {

    pub paths: Vec<Direction>,
    pub final_direction: Option<Direction>,

    #[serde(default)]
    pub current: usize,

}

#[cfg(feature = "pathfind")]
impl DestinationPath {

    pub fn new(start: Position, destination: Destination, world: &impl World) -> Option<Self> {
        astar::pathfind(start, destination.coords, world).map(|paths| 
            Self {
                paths,
                final_direction: destination.direction,
                current: 0,
            }
        )
             
    }

}