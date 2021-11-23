use serde::{Deserialize, Serialize};
use tinystr::TinyStr8;
use std::collections::HashSet;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MapData {

    #[serde(default)]
    pub battled: HashSet<TinyStr8>,

    // #[serde(default)]
    // pub npcs: HashMap<u8, bool>, // npc states, active / not active

}