use serde::{Deserialize, Serialize};
use util::{Direction, Destination};
use font::message::{Message, MessagePages};
use audio::{music::MusicName, sound::Sound};

use pokedex::{
    pokemon::instance::PokemonInstance,
    item::ItemId,
};

use crate::{
    character::npc::{NPC, NPCId},
    map::warp::{WarpId, WarpDestination}
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldActionKind {

    PlayMusic(MusicName),
    PlayMapMusic,
    PlaySound(Sound),


    PlayerFreezeInput,
    PlayerUnfreezeInput,
    PlayerUnfreeze,

    PlayerLook(Direction),
    PlayerMove(Destination),

    PlayerGivePokemon(PokemonInstance),
    PlayerHealPokemon,

    PlayerGiveItem(ItemId),


    NPCAdd(NPCId, NPC),
    NPCRemove(NPCId),
    // NPCSpawn(NPCId),
    // NPCDespawn(NPCId),

    NPCLook(NPCId, Direction),
    NPCMove(NPCId, Destination),

    NPCLeadPlayer(NPCId, Destination),
    NPCMoveToPlayer(NPCId),

    NPCInteract(NPCId),
    NPCSay(NPCId, MessagePages),
    NPCBattle(NPCId),


    Info(String),
    // Warn(String),

    Wait(f32),

    DisplayText(Message),
    
    Conditional {
        // #[deprecated]
        message: Message,

        #[serde(default)] end_message: Option<Message>,
        // false_next: Vec<WorldActionKind>,
        #[serde(default = "def_true")]
        unfreeze: bool,
    }, // yes or no box, no despawns the script after an optional message, and bool unfreezes player if true,

    Warp(ScriptWarp), // bool: change music

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScriptWarp {
    Id(WarpId),
    Dest(WarpDestination),
    KeepMusic(WarpId),
}

const fn def_true() -> bool {
    true
}