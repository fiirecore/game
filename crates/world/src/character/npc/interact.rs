use crate::script::ScriptId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum NpcInteract {
    Nothing,
    Script(ScriptId),
}

impl Default for NpcInteract {
    fn default() -> Self {
        Self::Nothing
    }
}

impl NpcInteract {
    pub fn is_some(&self) -> bool {
        !matches!(self, Self::Nothing)
    }
}
