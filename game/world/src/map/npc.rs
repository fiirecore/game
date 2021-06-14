use serde::{Deserialize, Serialize};
use deps::hash::HashMap;
use crate::character::npc::{Npc, NpcId};

pub type NpcMap = HashMap<NpcId, Option<Npc>>;
pub type ActiveNpc = Option<(NpcId, Npc)>;

#[derive(Default, Serialize, Deserialize)]
pub struct NpcManager {
    pub list: NpcMap,
    pub active: ActiveNpc,
}

impl NpcManager {
    pub fn get(&self, id: &NpcId) -> Option<&Npc> {
        self.list.get(id).map(|npc| npc.as_ref()).unwrap_or_else(|| self.active.as_ref().filter(|(active, _)| active == id).map(|(_, npc)| npc))
    }
    pub fn get_mut(&mut self, id: &NpcId) -> Option<&mut Npc> {
        self.list.get_mut(id).map(|npc| npc.as_mut()).unwrap_or(self.active.as_mut().filter(|(active, _)| active == id).map(|(_, npc)| npc))
    }
}

impl From<NpcMap> for NpcManager {
    fn from(list: NpcMap) -> Self {
        NpcManager {
            list,
            active: Default::default(),
        }
    }
}