use worldlib::character::{
    npc::group::{NpcGroup, TrainerGroup, TrainerGroupId},
    CharacterGroupId, CharacterState,
};

use crate::engine::utils::HashMap;

// pub type NpcTypeMap = HashMap<NpcTypeId, NpcType>;

// #[derive(Debug, Clone)]
// pub struct NpcTypes(NpcTypeMap);

pub fn group<'a>(
    group: &'a HashMap<CharacterGroupId, NpcGroup>,
    id: &CharacterGroupId,
) -> &'a NpcGroup {
    group.get(id).unwrap_or_else(|| {
        group
            .get(&CharacterState::PLACEHOLDER)
            .unwrap_or_else(|| panic!("Cannot get placeholder npc type!"))
    })
}

pub fn trainer<'a>(
    group: &'a HashMap<TrainerGroupId, TrainerGroup>,
    id: &TrainerGroupId,
) -> &'a TrainerGroup {
    group.get(id).unwrap_or_else(|| {
        group
            .get(&TrainerGroup::PLACEHOLDER)
            .unwrap_or_else(|| panic!("Cannot get placeholder trainer type!"))
    })
}
