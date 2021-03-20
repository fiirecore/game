use serde::{Deserialize, Serialize};
use ahash::AHashSet as HashSet;

use firecore_world::character::npc::{NPC, NPCId};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MapData {

    pub battled: HashSet<String>,
    pub disabled_npcs: HashSet<NPCId>,

}

impl MapData {

    pub fn battle(&mut self, npc: &NPC) {
        if !self.battled.contains(&npc.identifier.name) {
            if npc.trainer.is_some() {
                crate::util::battle_data::trainer_battle(&npc);
                self.battled.insert(npc.identifier.name.clone());
                for name in &npc.trainer.as_ref().unwrap().disable_others {
                    self.battled.insert(name.clone());
                }
            }
        } else {
            macroquad::prelude::info!("Player has already battled {}", npc.identifier.name);
        }
    }

}