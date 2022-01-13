use crate::positions::{BoundingBox, Direction};
use serde::{Deserialize, Serialize};

use crate::script::ScriptId;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Condition {
    Location(BoundingBox),
    Activate(Direction), // make stuff activate on a click, not look
    NoRepeat,
    Script(ScriptId, bool), // bool = has happened
    PlayerHasPokemon(bool),
}