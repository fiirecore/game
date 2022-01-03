use serde::{Deserialize, Serialize};

mod condition;
mod instructions;

pub use self::condition::Condition;
pub use self::instructions::*;

use super::ScriptId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptEnvironment {
    pub queue: Vec<WorldInstruction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldScript {
    pub identifier: ScriptId,
    pub actions: Vec<WorldInstruction>,
}