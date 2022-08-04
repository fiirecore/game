use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonObjectEvent {
    pub graphics_id: String,
    pub x: i32,
    pub y: i32,
    pub elevation: u8,
    pub movement_type: String,
    pub movement_range_x: u8,
    pub movement_range_y: u8,
    pub trainer_type: String,
    pub trainer_sight_or_berry_tree_id: String,
    pub script: String,
    pub flag: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonBgEvent {
    #[serde(rename = "type")]
    pub type_: String,
    pub x: i32,
    pub y: i32,
    pub elevation: u8,
    // Item section
    pub item: Option<String>,
    pub flag: Option<String>,
    pub quantity: Option<usize>,
    pub underfoot: Option<bool>,
    // Sign section
    pub player_facing_dir: Option<String>,
    pub script: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonCoordEvent {
    #[serde(rename = "type")]
    pub type_: String,
    pub x: i32,
    pub y: i32,
    pub elevation: u8,
    pub var: String,
    pub var_value: String,
    pub script: String,
}