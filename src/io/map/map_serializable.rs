use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MapConfig {

    pub identifier: MapIdentifier,

    pub jigsaw_map: Option<SerializedChunkMap>,
    pub warp_map: Option<SerializedMapSet>,

    //pub settings: Option<TomlMapSettings>,
    pub wild: Option<SerializedWildEntry>,

}

impl MapConfig {

    pub fn from_string(data: &str) -> Result<MapConfig, toml::de::Error> {
        toml::from_str(data)
    }

}

#[derive(Debug, Deserialize)]
pub struct MapIdentifier {

    #[serde(default = "map_default_name")]
    pub name: String,
    pub map_files: Vec<String>,

}

fn map_default_name() -> String {
    String::from("Map (Missing Name)")
}

// impl MapIdentifier {

//     pub fn name(&self) -> String {
//         if let Some(name) = &self.name {
//             return name.clone();
//         } else {
//             return String::from("Map (Missing Name)");
//         }
//     }

// }

#[derive(Debug, Deserialize)]
pub struct SerializedChunkMap {

    pub piece_index: u16,
    pub x: isize,
    pub y: isize,
    pub connections: Vec<u16>,

}

#[derive(Debug, Deserialize)]
pub struct SerializedMapSet {

    pub map_set_id: String,

}

#[derive(Debug, Deserialize, Clone)]
pub struct SerializedWildEntry {

    pub encounter_type: String,
    pub wild_tiles: Option<Vec<u16>>,
    
}