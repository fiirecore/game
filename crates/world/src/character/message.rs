use super::{player::PlayerCharacter, Character};

pub const SELF_ID: &str = "%s";
pub const PLAYER_ID: &str = "%p";
pub const RIVAL_ID: &str = "%r";

// pub fn process_messages(pages: &Vec<Vec<String>>, character: &Character) -> Vec<Vec<String>> {
//     //, npc: Option<&Character>) {, npc: Option<&Character>) {
//     pages
//         .iter()
//         .map(|page| {
//             page.iter()
//                 .map(|line| {
//                     process_string(line, character)
//                 })
//                 .collect()
//         })
//         .collect()
// }

pub fn process_str(string: &str, character: &Character) -> String {
    if string.contains(SELF_ID) {
        string.replace(SELF_ID, &character.name)
    } else {
        string.to_owned()
    }
}

pub fn process_str_player(string: &str, player: &PlayerCharacter) -> String {
    let mut string = string.to_owned();
    if string.contains(PLAYER_ID) {
        string = string.replace(PLAYER_ID, &player.name);
    }
    if string.contains(RIVAL_ID) {
        string = string.replace(RIVAL_ID, &player.rival);
    }
    string
}
