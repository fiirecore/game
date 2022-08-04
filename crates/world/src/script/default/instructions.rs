use audio::{SoundId, SoundVariant};
use pokedex::item::ItemId;
use serde::{Deserialize, Serialize};

use crate::{map::object::ObjectId, positions::Direction, character::npc::NpcMovement};

use super::{Flag, MessageId, ScriptId, Variable, VariableName};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorldInstruction {
    /// End script execution
    End,
    /// Does nothing?? maybe just try to end script when no instructions are left
    Return,

    /// Set variable
    SetVar(VariableName, Variable),

    /// Set flag
    SetFlag(Flag),

    /// Set variable using special method
    SpecialVar(VariableName, String),

    /// Compare variable to given variable
    Compare(VariableName, Variable),

    /// Call function (and return back to this one)
    Call(ScriptId),

    /// Goto if "EQ" variable is true,
    GotoIfEq(ScriptId),

    /// Goto script if set
    GotoIfSet(VariableName, ScriptId),

    /// 0x4F
    /// Applies the movement data at movements to the specified (index) Person event. Also closes any standard message boxes that are still open.

    /// Indices 0xFF and 0x7F refer to the player and the camera, respectively. 
    /// Running this command from a Script event will crash the game unless that Script event's "Unknown" field (in AdvanceMap) is "$0003" and its "Var number" field refers to a valid script variable.
    ApplyMovement(ObjectId, Vec<(Direction, bool)>),

    /// 0x51
    ///
    WaitMovement(ObjectId),

    /// 0x5A
    /// Makes executor NPC face player
    FacePlayer,

    /// 0x69
    /// Ceases movement for all OWs on-screen.
    LockAll,
    /// 0x6A
    /// If the script was called by a Person event, then that Person's movement will cease.
    Lock,
    /// 0x6B
    /// Resumes normal movement for all OWs on-screen, and closes any standard message boxes that are still open.
    ReleaseAll,
    /// 0x6C
    /// If the script was called by a Person event, then that Person's movement will resume.
    /// This command also closes any standard message boxes that are still open.
    Release,
    
    /// Npc walks in a direction
    Walk(Direction),

    Look(Direction),

    /// Start trainer battle
    TrainerBattleSingle,

    /// Runs message with ID
    Msgbox(MessageId, Option<String>),

    /// Sets text color for [WorldInstruction::Message] command
    TextColor(u8),
    /// Different from message box command, runs message with id
    Message(MessageId),
    WaitMessage,

    PlayFanfare(SoundId, SoundVariant),
    WaitFanfare(),

    /// Give player an item
    AddItem(ItemId),
    CheckItemSpace(String, i32), //ItemId)
    GetItemName(i32, String),
}
