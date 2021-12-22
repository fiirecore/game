use crate::{map::warp::WarpId, positions::{CoordinateInt, Direction, Location, Position}, script::ScriptId};
use serde::{Deserialize, Serialize};

use pokedex::{item::SavedItemStack, pokemon::owned::SavedPokemon};
use tinystr::TinyStr8;

use crate::{
    character::npc::{Npc, NpcId},
    map::{warp::WarpDestination, MusicId},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldAction {
    PlayMusic(MusicId),
    PlayMapMusic,
    PlaySound(TinyStr8, Option<u16>),

    PlayerFreezeInput,
    PlayerUnfreezeInput,

    // PlayerIgnoreTiles,

    PlayerLook(Direction),
    PlayerMove(CoordinateInt, CoordinateInt),

    PlayerGivePokemon(SavedPokemon), //, bool),
    PlayerHealPokemon,

    PlayerGiveItem(SavedItemStack),

    NpcAdd(NpcId, Box<Npc>),
    NpcRemove(NpcId),
    NpcLook(NpcId, Direction),
    NpcMove(NpcId, CoordinateInt, CoordinateInt),

    NpcLeadPlayer(NpcId, CoordinateInt, CoordinateInt),
    NpcMoveToPlayer(NpcId),

    NpcInteract(NpcId),
    NpcSay(NpcId, Vec<Vec<String>>, #[serde(default = "crate::default_true")] bool),
    NpcBattle(NpcId),
    NpcWarp(NpcId, NpcWarp),

    // Info(String),
    // Warn(String),
    Wait(f32),
    WaitMessage,
    WaitFinishWarp,

    // Conditional {
    //     // #[deprecated]
    //     message: Message,

    //     #[serde(default)]
    //     end_message: Option<Message>,
    //     // false_next: Vec<WorldActionKind>,
    //     #[serde(default = "def_true")]
    //     unfreeze: bool,
    // }, // yes or no box, no despawns the script after an optional message, and bool unfreezes player if true,

    Warp(PlayerWarp, bool), // bool = keep music
    Finish(ScriptId),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerWarp {
    Id(WarpId),
    Dest(WarpDestination),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NpcWarp {
    Id(WarpId),
    Dest(Location, Position),
}