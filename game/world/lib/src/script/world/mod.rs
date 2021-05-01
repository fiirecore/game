use serde::{Deserialize, Serialize};
use firecore_util::{Entity, Timer, BoundingBox, Coordinate};

use std::collections::VecDeque;

mod condition;
mod actions;

pub use self::condition::Condition;
pub use self::actions::*;

use super::ScriptId;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldScript {
    
    pub identifier: ScriptId,

    pub location: Option<BoundingBox>,

    pub conditions: Vec<Condition>,

    #[serde(rename = "actions")]
    original_actions: VecDeque<WorldActionKind>,

    #[serde(skip)]
    pub actions: VecDeque<WorldActionKind>, // clones actions to this so scripts can be reused as the main actions field does not use up its values

    #[serde(skip)]
    alive: bool, // script is running or not

    #[serde(skip)]
    pub option: u8, // variable to be used by script for persistant data in update loop (used in ConditionOrBreak)

    #[serde(skip)]
    pub timer: Timer, // timer for script waiting events

}

impl WorldScript {

    fn on_spawn(&mut self) {
        self.actions = self.original_actions.clone();
    }

    pub fn in_location(&self, coords: &Coordinate) -> bool {
        self.location.as_ref().map(|location| location.in_bounds(coords)).unwrap_or(true)
    }

}

impl Entity for WorldScript {
    fn spawn(&mut self) {
        self.alive = true;
        self.on_spawn();
    }

    fn despawn(&mut self) {
        self.alive = false;
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}