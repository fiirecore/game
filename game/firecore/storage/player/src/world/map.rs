use serde::{Deserialize, Serialize};
use firecore_util::hash::{HashMap, HashSet};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MapData {

    #[serde(default)]
    pub battled: HashSet<u8>,

    #[serde(default)]
    pub npcs: HashMap<u8, bool>, // npc states, active / not active

}