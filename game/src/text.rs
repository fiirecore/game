use util::text::Message;
use data::player::PlayerSave;

pub fn process_messages(player_save: &PlayerSave, messages: &mut Vec<Message>) {
    for message in messages.iter_mut() {
        for message in &mut message.lines {
            *message = message
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