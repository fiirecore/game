use firecore_battle_gui::pokedex::engine::graphics::Color;
use std::collections::HashMap;
use worldlib::character::npc::{MessageColor, NpcType, NpcTypeId};

pub type NpcTypes = HashMap<NpcTypeId, NpcType>;

pub fn color(message: &MessageColor) -> Color {
    match message {
        MessageColor::Black => Color::rgb(20.0 / 255.0, 20.0 / 255.0, 20.0 / 255.0),
        MessageColor::White => Color::rgb(240.0 / 255.0, 240.0 / 255.0, 240.0 / 255.0),
        MessageColor::Red => Color::rgb(0.90, 0.16, 0.22),
        MessageColor::Blue => Color::rgb(48.0 / 255.0, 80.0 / 255.0, 200.0 / 255.0),
    }
}
