use serde::{Deserialize, Serialize};
use deps::hash::HashMap;
use crate::character::npc::{NPC, NPCId};

pub type NPCMap = HashMap<NPCId, Option<NPC>>;
pub type ActiveNPC = Option<(NPCId, NPC)>;

#[derive(Default, Serialize, Deserialize)]
pub struct NPCManager {
    pub list: NPCMap,
    pub active: ActiveNPC,
}

impl NPCManager {
    pub fn get(&self, id: &NPCId) -> Option<&NPC> {
        self.list.get(id).map(|npc| npc.as_ref()).unwrap_or(self.active.as_ref().filter(|(active, _)| active == id).map(|(_, npc)| npc))
    }
    pub fn get_mut(&mut self, id: &NPCId) -> Option<&mut NPC> {
        self.list.get_mut(id).map(|npc| npc.as_mut()).unwrap_or(self.active.as_mut().filter(|(active, _)| active == id).map(|(_, npc)| npc))
    }
}

impl Into<NPCManager> for NPCMap {
    fn into(self) -> NPCManager {
        NPCManager {
            list: self,
            active: Default::default(),
        }
    }
}