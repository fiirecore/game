use serde::{Deserialize, Serialize};

mod condition;
mod actions;

pub use self::condition::Condition;
pub use self::actions::*;

use super::ScriptId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldScript {
    pub identifier: ScriptId,
    pub conditions: Vec<Condition>,
    pub actions: Vec<WorldAction>,
}