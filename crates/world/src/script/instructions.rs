use serde::{Deserialize, Serialize};

use crate::{positions::Direction, script::ScriptId};

use super::{MessageId, Variable, VariableName};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorldInstruction {
    /// End script execution
    End,
    /// Does nothing?? maybe just try to end script when no instructions are left
    Return,

    /// Set variable
    SetVar(VariableName, Variable),

    /// Set variable using special method
    SpecialVar(VariableName, String),

    /// Compare variable to given variable
    Compare(VariableName, Variable),

    /// Goto if "EQ" variable is true,
    GotoIfEq(ScriptId),

    /// Goto script if set
    GotoIfSet(VariableName, ScriptId),

    /// Freezes player
    Lock,
    /// Unfreezes player
    Release,
    /// Makes executor NPC face player
    FacePlayer,
    /// Npc walks in a direction
    Walk(Direction),

    /// Start trainer battle
    TrainerBattleSingle,

    /// Runs message with ID
    Msgbox(MessageId, Option<String>),
}
