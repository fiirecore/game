pub use firecore_text::message::*;
use storage::player::PlayerSave;

pub fn process_messages(player_save: &PlayerSave, message: &mut Message) {
    for message in message.message_set.iter_mut() {
        for lines in message.lines.iter_mut() {
            *lines = lines
                .replace("%r", rival_name())
                .replace("%p", player_name(player_save))
            ;
        }
    }
}

pub fn player_name(player_save: &PlayerSave) -> &String {
    &player_save.name
}

pub fn rival_name() -> &'static str {
    "Gary"
}