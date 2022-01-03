use serde::{Deserialize, Serialize};

use crate::{character::npc::NpcId, script::ScriptId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldInstruction {
    TrainerBattleSingle(NpcId),
    SpecialVar(Variable, WorldFunction),
    Compare(Variable, u8),
    GotoIfEq(ScriptId),
    Msgbox()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Variable {
    Result,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorldFunction {
    ShouldTryRematchBattle,
}
