use serde::Deserialize;
use util::{Coordinate, Location, LocationId};
use worldlib::map::WorldChunk;

pub mod map;
pub mod textures;
// pub mod constants;

pub mod wild;
pub mod warp;
pub mod npc;
pub mod script;
// pub mod mart;


#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MapConfig {

    // #[deprecated(note = "use full location")]
    pub identifier: LocationId,
    pub name: String,
    pub file: String,

    #[serde(default)]
    pub chunk: Option<SerializedChunk>,

    #[serde(default)]
    pub settings: SerializedMapSettings,

    #[serde(default)]
    pub pokemon_center: bool,

}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SerializedChunk {

    pub coords: Coordinate,
    pub connections: Vec<LocationId>,
    #[serde(default)]
    pub map_position: Option<firecore_dependencies::tetra::math::Vec2<u8>>,

}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SerializedMapList {

    // #[deprecated(note = "remove")]
    pub identifier: LocationId,
    pub dirs: Vec<String>,

}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SerializedMapSettings {

    #[serde(default)]
    pub fly_position: Option<Coordinate>,
    #[serde(default)]
    pub time: worldlib::map::WorldTime,

}

impl From<SerializedChunk> for WorldChunk {
    fn from(chunk: SerializedChunk) -> Self {
        WorldChunk {
            coords: chunk.coords,
            connections: chunk.connections.into_iter().map(|index| Location::new(None, index)).collect()
        }
    }
}