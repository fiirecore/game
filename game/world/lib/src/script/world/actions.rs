use firecore_font::message::MessageSet;
use serde::{Deserialize, Serialize};

use firecore_util::{Direction, Destination};
use firecore_font::message::Message;

use firecore_audio_lib::music::MusicName;
use firecore_audio_lib::sound::Sound;

use firecore_pokedex::pokemon::saved::SavedPokemon;

use firecore_pokedex::item::ItemId;

use crate::character::npc::{NPC, NPCId};
use crate::map::warp::WarpDestination;

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

    PlayerGivePokemon(SavedPokemon),
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
    NPCSay(NPCId, MessageSet),
    NPCBattle(NPCId),


    Info(String),
    // Warn(String),

    Wait(f32),

    DisplayText(Message),
    
    Conditional {
        message: Message,

        #[serde(default)] end_message: Option<Message>,
        // false_next: Vec<WorldActionKind>,
        #[serde(default = "def_true")]
        unfreeze: bool,
    }, // yes or no box, no despawns the script after an optional message, and bool unfreezes player if true,

    Warp(WarpDestination, #[serde(default = "def_true")] bool), // bool: change music

}

const fn def_true() -> bool {
    true
}