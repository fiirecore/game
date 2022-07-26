use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonWarpEvent {
    pub x: isize,
    pub y: isize,
    pub elevation: u8,
    #[serde(rename = "dest_map")]
    pub destination: String,
    pub dest_warp_id: u8,
}
