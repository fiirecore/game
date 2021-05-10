use serde::Deserialize;
use util::Coordinate;
use worldlib::map::{MapIdentifier, chunk::Connections};

pub mod map;
pub mod textures;

pub mod wild;
pub mod warp;
pub mod npc;
pub mod script;


#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MapConfig {

    pub identifier: MapIdentifier,
    pub name: String,
    pub file: String,

    #[serde(default)]
    pub settings: SerializedMapSettings,

}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SerializedChunkMap {

    pub config: MapConfig,

    pub coords: Coordinate,
    pub connections: Connections,

}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SerializedMapSet {

    pub identifier: MapIdentifier,
    pub dirs: Vec<String>,

}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SerializedMapSettings {

    pub fly_position: Option<Coordinate>,

}