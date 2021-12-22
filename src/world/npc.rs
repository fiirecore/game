use firecore_battle_gui::pokedex::engine::graphics::Color;
use std::collections::HashMap;
use worldlib::character::npc::{MessageColor, NpcType, NpcTypeId};

use super::map::data::npc::NpcData;

pub type NpcTypeMap = HashMap<NpcTypeId, NpcType>;

#[derive(Debug, Clone)]
pub struct NpcTypes(NpcTypeMap);

impl NpcTypes {
    pub fn new(map: NpcTypeMap) -> Self {
        Self(map)
    }

    pub fn get(&self, id: &NpcTypeId) -> &NpcType {
        self.0.get(id).unwrap_or_else(|| {
            self.0
                .get(&NpcData::PLACEHOLDER)
                .unwrap_or_else(|| panic!("Cannot get placeholder npc type!"))
        })
    }
}

pub fn color(message: &MessageColor) -> Color {
    match message {
        MessageColor::Black => Color::rgb(20.0 / 255.0, 20.0 / 255.0, 20.0 / 255.0),
        MessageColor::White => Color::rgb(240.0 / 255.0, 240.0 / 255.0, 240.0 / 255.0),
        MessageColor::Red => Color::rgb(0.90, 0.16, 0.22),
        MessageColor::Blue => Color::rgb(48.0 / 255.0, 80.0 / 255.0, 200.0 / 255.0),
    }
}
