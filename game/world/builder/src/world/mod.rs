use serde::Deserialize;
use util::Coordinate;
use worldlib::map::{MapIdentifier, chunk::Connections};

pub mod map;
pub mod textures;

pub mod wild;
pub mod warp;
pub mod npc;
pub mod script;


#[serde(deny_unknown_fields)]
#[derive(Deserialize)]
pub struct MapConfig {

    pub identifier: MapIdentifier,
    pub name: String,
    pub file: String,

    #[serde(default)]
    pub settings: SerializedMapSettings,

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