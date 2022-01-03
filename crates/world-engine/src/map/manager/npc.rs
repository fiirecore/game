use worldlib::character::npc::group::{MessageColor, NpcGroup, NpcGroupId};

use crate::engine::{graphics::Color, utils::HashMap};

// pub type NpcTypeMap = HashMap<NpcTypeId, NpcType>;

// #[derive(Debug, Clone)]
// pub struct NpcTypes(NpcTypeMap);

pub fn group<'a>(group: &'a HashMap<NpcGroupId, NpcGroup>, id: &NpcGroupId) -> &'a NpcGroup {
    group.get(id).unwrap_or_else(|| {
        group
            .get(&NpcGroup::PLACEHOLDER)
            .unwrap_or_else(|| panic!("Cannot get placeholder npc type!"))
    })
}

pub fn color(message: &MessageColor) -> Color {
    match message {
        MessageColor::Black => Color::rgb(20.0 / 255.0, 20.0 / 255.0, 20.0 / 255.0),
        MessageColor::White => Color::rgb(240.0 / 255.0, 240.0 / 255.0, 240.0 / 255.0),
        MessageColor::Red => Color::rgb(0.90, 0.16, 0.22),
        MessageColor::Blue => Color::rgb(48.0 / 255.0, 80.0 / 255.0, 200.0 / 255.0),
    }
}
