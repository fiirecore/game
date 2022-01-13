use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

mod condition;
mod instructions;

use crate::{character::npc::NpcId};

pub use self::condition::Condition;
pub use self::instructions::*;


pub type ScriptId = String;
pub type MessageId = String;

pub type VariableName = String;
pub type Variable = u16;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldScriptData {
    pub scripts: HashMap<ScriptId, Vec<WorldInstruction>>,
    pub messages: HashMap<MessageId, Vec<Vec<String>>>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ScriptEnvironment {
    pub executor: Option<NpcId>,
    pub queue: Vec<WorldInstruction>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptFlag {
    /// Flag that shows that a variable exists
    Flag,
    Var(u16),
}

impl ScriptEnvironment {

    pub fn running(&self) -> bool {
        self.executor.is_some() || !self.queue.is_empty()
    }

}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct WorldScript {
//     pub identifier: ScriptId,
//     pub actions: Vec<WorldInstruction>,
// }