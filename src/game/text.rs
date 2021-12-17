use crate::engine::gui::MessagePage;
use crate::saves::PlayerData;

const PLAYER_ID: &str = "%p";
const RIVAL_ID: &str = "%r";

pub fn process_messages(pages: &mut [MessagePage], save: &PlayerData) {
    for page in pages {
        for lines in page.lines.iter_mut() {
            process_string(lines, save);
        }
    }
}

pub fn process_string(string: &mut String, save: &PlayerData) {
    if string.contains(PLAYER_ID) {
        *string = string.replace("%p", player_name(save));
    }
    if string.contains(RIVAL_ID) {
        *string = string.replace("%r", rival_name());
    }
}

pub fn player_name<'d>(player_save: &'d PlayerData) -> &'d str {
    &player_save.name
}

pub fn rival_name() -> &'static str {
    "Gary"
}