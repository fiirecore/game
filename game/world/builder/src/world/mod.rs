use firecore_world_lib::map::MapIdentifier;
use firecore_world_lib::map::chunk::Connections;
use serde::Deserialize;

use firecore_util::Coordinate;

pub mod map;

pub mod wild;
pub mod warp;
pub mod npc;
pub mod script;

#[derive(Deserialize)]
pub struct MapConfig {

    pub identifier: MapIdentifier,
    pub name: String,
    pub file: String,

    #[serde(default)]
    pub settings: SerializedMapSettings,
    pub wild: Option<SerializedWildEntry>,

}

#[serde(deny_unknown_fields)]
#[derive(Deserialize)]
pub struct SerializedChunkMap {

    pub config: MapConfig,

    pub coords: Coordinate,
    pub connections: Connections,

}

#[serde(deny_unknown_fields)]
#[derive(Deserialize)]
pub struct SerializedMapSet {

    pub identifier: MapIdentifier,
    pub dirs: Vec<String>,

}

#[derive(Default, Deserialize)]
pub struct SerializedMapSettings {

    pub fly_position: Option<Coordinate>,

}

#[derive(Deserialize, Clone)]
pub struct SerializedWildEntry {

    #[serde(rename = "type")]
    pub encounter_type: String,
    #[serde(default)]
    pub tiles: Option<Vec<u16>>,
    
}