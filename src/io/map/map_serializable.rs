use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MapConfig {

    pub identifier: MapIdentifier,

    pub jigsaw_map: Option<TomlJigsawMap>,
    pub warp_map: Option<TomlWarpMap>,

    pub settings: Option<TomlMapSettings>,
    pub wild: Option<TomlWildEntry>,

}

#[derive(Debug, Deserialize)]
pub struct MapIdentifier {

    pub world_id: String,
    pub map_files: Vec<String>,

    pub name: Option<String>,

}

impl MapIdentifier {

    pub fn name(&self) -> String {
        if let Some(name) = &self.name {
            return name.clone();
        } else {
            return String::from("Map");
        }
    }

}

#[derive(Clone, Debug, Deserialize)]
pub struct TomlJigsawMap {

    pub piece_index: u16,
    pub x: isize,
    pub y: isize,
    pub connections: Vec<u16>,

}

#[derive(Debug, Deserialize)]
pub struct TomlWarpMap {

    pub map_set_id: String,

}

#[derive(Debug, Deserialize)]
pub struct TomlMapSettings {

}

#[derive(Debug, Deserialize)]
pub struct TomlWildEntry {

    pub encounter_type: String,
    pub wild_tiles: Vec<u16>,
    
}