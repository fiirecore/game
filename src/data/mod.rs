pub mod map;

pub mod player {
    pub static mut DIRTY: bool = false;

    use firecore_data::player::world::map::MapData;
    pub use firecore_data::player::{list, save, world};
    use firecore_world::character::npc::NPC;

    use crate::world::NPCTypes;

    pub fn battle(data: &mut MapData, npc: &NPC, npc_types: &NPCTypes) {
        if !data.battled.contains(&npc.identifier.index) {
            if npc.trainer.is_some() {
                crate::battle::data::trainer_battle(&npc, npc_types);
                data.battled.insert(npc.identifier.index.clone());
                for index in &npc.trainer.as_ref().unwrap().disable_others {
                    data.battled.insert(*index);
                }
            }
        } else {
            macroquad::prelude::info!("Player has already battled {}", npc.identifier.name);
        }
    }

}
