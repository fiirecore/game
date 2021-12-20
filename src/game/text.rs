use firecore_world::character::{player::PlayerCharacter, Character};

use crate::engine::text::MessagePage;

const PLAYER_ID: &str = "%p";
const RIVAL_ID: &str = "%r";
const NPC_ID: &str = "%n";

pub fn process_messages(pages: &mut [MessagePage], player: &PlayerCharacter, npc: Option<&Character>) {
    for page in pages {
        for lines in page.lines.iter_mut() {
            process_string(lines, player, npc);
        }
    }
}

pub fn process_string(string: &mut String, player: &PlayerCharacter, npc: Option<&Character>) {
    if string.contains(PLAYER_ID) {
        *string = string.replace(PLAYER_ID, &player.name);
    }
    if string.contains(RIVAL_ID) {
        *string = string.replace(RIVAL_ID, &player.rival);
    }
    if let Some(npc) = npc {
        if string.contains(NPC_ID) {
            *string = string.replace(NPC_ID, &npc.name);
        }
    }
}