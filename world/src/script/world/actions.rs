use crate::{map::warp::WarpId, positions::{CoordinateInt, Direction, Location, Position}, script::ScriptId};
use audio::{music::MusicName, sound::Sound};
use font::message::{Message, MessagePages};
use serde::{Deserialize, Serialize};

use pokedex::{item::ItemId, pokemon::instance::PokemonInstance};

use crate::{
    character::npc::{Npc, NpcId},
    map::warp::WarpDestination,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldAction {
    PlayMusic(MusicName),
    PlayMapMusic,
    PlaySound(Sound),

    PlayerFreezeInput,
    PlayerUnfreezeInput,

    // PlayerIgnoreTiles,

    PlayerLook(Direction),
    PlayerMove(CoordinateInt, CoordinateInt),

    PlayerGivePokemon(PokemonInstance), //, bool),
    PlayerHealPokemon,

    PlayerGiveItem(ItemId),

    NpcAdd(NpcId, Box<Npc>),
    NpcRemove(NpcId),
    // NpcSpawn(NpcId),
    // NpcDespawn(NpcId),
    NpcLook(NpcId, Direction),
    NpcMove(NpcId, CoordinateInt, CoordinateInt),

    NpcLeadPlayer(NpcId, CoordinateInt, CoordinateInt),
    NpcMoveToPlayer(NpcId),

    NpcInteract(NpcId),
    NpcSay(NpcId, MessagePages),
    NpcBattle(NpcId),
    NpcWarp(NpcId, NpcWarp),

    // Info(String),
    // Warn(String),
    Wait(f32),
    WaitFinishWarp,

    DisplayText(Message),

    Conditional {
        // #[deprecated]
        message: Message,

        #[serde(default)]
        end_message: Option<Message>,
        // false_next: Vec<WorldActionKind>,
        #[serde(default = "def_true")]
        unfreeze: bool,
    }, // yes or no box, no despawns the script after an optional message, and bool unfreezes player if true,

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

const fn def_true() -> bool {
    true
}
