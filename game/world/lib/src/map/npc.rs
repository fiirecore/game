use firecore_util::Timer;
use serde::{Deserialize, Serialize};
use deps::hash::HashMap;

use crate::character::movement::MovementType;
use crate::character::npc::NPC;
use crate::character::npc::NPCId;

pub type NPCMap = HashMap<NPCId, NPC>;

#[derive(Deserialize, Serialize)]
pub struct NPCManager {

    pub npcs: NPCMap,

    #[serde(skip)]
    pub active: Option<NPCId>,

    #[serde(skip, default = "default_npc_timer")]
    pub timer: Timer,

}

impl NPCManager {

    pub fn new(npcs: NPCMap) -> Self {
        Self {
            npcs,
            ..Default::default()
        }
    }

    pub fn get(&self, id: &NPCId) -> Option<&NPC> {
        self.npcs.get(id)
    }

    pub fn get_mut(&mut self, id: &NPCId) -> Option<&mut NPC> {
        self.npcs.get_mut(id)//.filter(|npc| npc.is_alive())
    }

    pub fn do_move(&mut self, delta: f32) {
        for (index, npc) in self.npcs.iter_mut().filter(|(_, npc)| /*npc.is_alive() &&*/ npc.character.destination.is_some() && npc.movement != MovementType::Still) {
            if self.active.map(|active| active.ne(index)).unwrap_or(true) {
                npc.character.move_to_destination(delta);
            }            
        }
    }

}

impl Default for NPCManager {
    fn default() -> Self {
        Self {
            npcs: HashMap::new(),
            active: None,
            timer: default_npc_timer(),
        }
    }
}

pub fn default_npc_timer() -> Timer {
    Timer::new(0.5)
}