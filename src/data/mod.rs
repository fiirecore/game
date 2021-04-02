use std::sync::atomic::AtomicBool;

pub mod map;

pub static DIRTY: AtomicBool = AtomicBool::new(false); // if true, save player data

use firecore_data::player::world::map::MapData;
    pub use firecore_data::player::{PlayerSave, PlayerSaves, world};
    use firecore_world::character::npc::NPC;

    use crate::battle::data::BattleData;
    use crate::world::NPCTypes;

    pub fn battle(data: &mut MapData, battle_data: &mut Option<BattleData>,  npc: &NPC, npc_types: &NPCTypes) {
        if !data.battled.contains(&npc.identifier.index) {
            if npc.trainer.is_some() {
                crate::battle::data::trainer_battle(battle_data, &npc, npc_types);
                data.battled.insert(npc.identifier.index.clone());
                for index in &npc.trainer.as_ref().unwrap().disable_others {
                    data.battled.insert(*index);
                }
            }
        }// else {
        //     macroquad::prelude::info!("Player has already battled {}", npc.identifier.name);
        // }
    }
