pub mod map;
pub mod text;

pub mod player {
    pub static mut DIRTY: bool = false;

    pub use firecore_data_lib::player::{list, save, world};

    pub fn battle(data: &mut firecore_data_lib::player::world::map::MapData, npc: &firecore_world::character::npc::NPC) {
        if !data.battled.contains(&npc.identifier.index) {
            if npc.trainer.is_some() {
                crate::util::battle_data::trainer_battle(&npc);
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
