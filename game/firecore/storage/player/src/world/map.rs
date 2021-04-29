use serde::{Deserialize, Serialize};
use deps::{
    hash::HashSet,
    tinystr::TinyStr8,
};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct MapData {

    #[serde(default)]
    pub battled: HashSet<TinyStr8>,

    // #[serde(default)]
    // pub npcs: HashMap<u8, bool>, // npc states, active / not active

}