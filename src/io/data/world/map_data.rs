use serde::{Deserialize, Serialize};
use ahash::AHashSet as HashSet;

use firecore_world::character::npc::NPC;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MapData {

    pub battled: HashSet<String>,

}

impl MapData {

    pub fn battle(&mut self, npc: &NPC) {
        if !self.battled.contains(&npc.identifier.name) {
            if let Some(trainer) = npc.trainer.as_ref() {
                crate::util::battle_data::trainer_battle(&trainer, &npc.identifier.name, &npc.identifier.npc_type);
                self.battled.insert(npc.identifier.name.clone());
                for name in &trainer.disable_others {
                    self.battled.insert(name.clone());
                }
            }
        } else {
            macroquad::prelude::info!("Player has already battled {}", npc.identifier.name);
        }
    }

}