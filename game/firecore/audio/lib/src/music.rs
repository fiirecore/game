use serde::{Deserialize, Serialize};

pub type MusicId = u8;
pub type MusicName = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Music {

    pub track: MusicId,
    pub name: MusicName,
    #[serde(default)]
    pub data: MusicData,

}

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct MusicData {

    pub loop_start: Option<f64>,

}