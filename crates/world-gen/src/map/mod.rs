use serde::{Deserialize, Serialize};

pub mod object;
pub mod warp;
pub mod wild;

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonMap {
    pub data: JsonMapData,
    pub layout: JsonMapLayout,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonMapData {
    pub id: String,
    pub name: String,
    pub layout: String,
    pub music: String,
    pub region_map_section: String,
    pub requires_flash: bool,
    pub weather: String,
    pub map_type: String,
    pub allow_cycling: bool,
    pub allow_escaping: bool,
    pub allow_running: bool,
    pub show_map_name: bool,
    pub floor_number: isize,
    pub battle_scene: String,
    pub connections: Option<Vec<JsonConnection>>,
    pub object_events: Vec<object::JsonObjectEvent>,
    pub warp_events: Vec<warp::JsonWarpEvent>,
    pub coord_events: Vec<object::JsonCoordEvent>,
    pub bg_events: Vec<object::JsonBgEvent>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsonMapLayout {
    pub id: String,
    pub name: String,
    pub width: usize,
    pub height: usize,
    // border_width: usize,
    // border_height: usize,
    pub primary_tileset: String,
    pub secondary_tileset: String,

    pub border_filepath: String,
    pub blockdata_filepath: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonConnection {
    pub map: String,
    pub offset: isize,
    pub direction: String,
}

#[derive(Debug, Deserialize)]
pub struct JsonMapLayouts {
    pub layouts: Vec<LayoutOrNone>,
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct LayoutOrNone {
    #[serde(with = "either::serde_untagged")]
    pub inner: either::Either<JsonMapLayout, Nothing>,
}

#[derive(Debug, Deserialize)]
pub struct Nothing {}
