use worldlib::character::npc::group::{MessageColor, NpcGroup, NpcGroupId, TrainerGroupId, TrainerGroup};

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

pub fn trainer<'a>(group: &'a HashMap<TrainerGroupId, TrainerGroup>, id: &TrainerGroupId) -> &'a TrainerGroup {
    group.get(id).unwrap_or_else(|| {
        group
            .get(&TrainerGroup::PLACEHOLDER)
            .unwrap_or_else(|| panic!("Cannot get placeholder trainer type!"))
    })
}

pub fn color(message: MessageColor) -> Color {
    let a: [f32; 4] = message.into();
    a.into()
}
