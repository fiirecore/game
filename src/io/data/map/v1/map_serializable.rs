use serde::Deserialize;

use crate::util::Coordinate;

#[derive(Deserialize)]
pub struct MapConfig {

    pub identifier: MapIdentifier,

    pub jigsaw_map: Option<SerializedChunkMap>,
    pub warp_map: Option<SerializedMapSet>,

    #[serde(default)]
    pub settings: SerializedMapSettings,
    pub wild: Option<SerializedWildEntry>,

}

impl MapConfig {

    pub fn from_string(data: &str) -> Result<MapConfig, toml::de::Error> {
        toml::from_str(data)
    }

}

#[derive(Deserialize)]
pub struct MapIdentifier {

    #[serde(default = "map_default_name")]
    pub name: String,
    pub map_files: Vec<String>,

}

fn map_default_name() -> String {
    "Map (Missing Name)".to_owned()
}

#[derive(Default, Deserialize)]
pub struct SerializedMapSettings {

    pub fly_position: Coordinate,

}

#[derive(Deserialize)]
pub struct SerializedChunkMap {

    pub piece_index: u16,
    pub x: isize,
    pub y: isize,
    pub connections: Vec<u16>,

}

#[derive(Deserialize)]
pub struct SerializedMapSet {

    pub map_set_id: String,

}

#[derive(Deserialize, Clone)]
pub struct SerializedWildEntry {

    pub encounter_type: String,
    pub wild_tiles: Option<Vec<u16>>,
    
}