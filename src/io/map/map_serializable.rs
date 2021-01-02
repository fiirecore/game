use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MapConfig {

    pub identifier: MapIdentifier,

    pub jigsaw_map: Option<TomlJigsawMap>,
    pub warp_map: Option<TomlWarpMap>,

    pub settings: Option<TomlMapSettings>,

}

#[derive(Debug, Deserialize)]
pub struct MapIdentifier {

    pub world_id: String,
    pub map_files: Vec<String>,

}

#[derive(Clone, Debug, Deserialize)]
pub struct TomlJigsawMap {

    pub piece_index: usize,
    pub x: isize,
    pub y: isize,
    pub connections: Vec<usize>,

}

#[derive(Debug, Deserialize)]
pub struct TomlWarpMap {

    pub map_set_id: String,

}

#[derive(Debug, Deserialize)]
pub struct TomlMapSettings {

    pub encounter_type: Option<String>,
    pub wild_tiles: Option<Vec<u16>>,

}