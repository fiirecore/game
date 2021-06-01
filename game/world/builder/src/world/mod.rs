use serde::Deserialize;
use util::{Coordinate, LocationId};
use worldlib::map::WorldChunk;

pub mod map;
pub mod textures;

pub mod wild;
pub mod warp;
pub mod npc;
pub mod script;


#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MapConfig {

    #[deprecated(note = "use full location")]
    pub identifier: LocationId,
    pub name: String,
    pub file: String,

    #[serde(default)]
    pub chunk: Option<WorldChunk>,

    #[serde(default)]
    pub settings: SerializedMapSettings,

}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SerializedMapSet {

    #[deprecated(note = "remove")]
    pub identifier: LocationId,
    pub dirs: Vec<String>,

}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SerializedMapSettings {

    pub fly_position: Option<Coordinate>,

}