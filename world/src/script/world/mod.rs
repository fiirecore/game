use serde::{Deserialize, Serialize};
use crate::positions::{BoundingBox, Coordinate};

use std::collections::VecDeque;

mod condition;
mod actions;

pub use self::condition::Condition;
pub use self::actions::*;

use super::ScriptId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldScript {
    
    pub identifier: ScriptId,

    pub location: Option<BoundingBox>,

    pub conditions: Vec<Condition>,

    #[serde(rename = "actions")]
    pub(crate) original_actions: VecDeque<WorldActionKind>,

    #[serde(skip)]
    pub actions: VecDeque<WorldActionKind>, // clones actions to this so scripts can be reused as the main actions field does not use up its values

    #[serde(skip)]
    pub current: Option<WorldActionKind>,

    #[serde(skip)]
    pub(crate) alive: bool, // script is running or not

    #[serde(skip)]
    pub option: u8, // variable to be used by script for persistant data in update loop (used in ConditionOrBreak)

    #[deprecated]
    #[serde(skip)]
    pub counter: f32, // timer for script waiting events

}

impl WorldScript {

    pub fn in_location(&self, coords: &Coordinate) -> bool {
        self.location.as_ref().map(|location| location.in_bounds(coords)).unwrap_or(true)
    }

    pub fn spawn(&mut self) {
        self.alive = true;
        self.actions = self.original_actions.clone();
    }

    pub fn despawn(&mut self) {
        self.alive = false;
    }

    pub fn alive(&self) -> bool {
        self.alive
    }

}