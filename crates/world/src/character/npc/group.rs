use serde::{Deserialize, Serialize};

use crate::map::MusicId;

pub type NpcGroupId = tinystr::TinyStr16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcGroup {
    pub message: MessageColor,
    pub trainer: Option<TrainerGroup>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum MessageColor {
    Black,
    White,
    Red,
    Blue,
}

impl Default for MessageColor {
    fn default() -> Self {
        Self::Black
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TrainerGroup {
    pub name: String,
    pub music: Option<MusicId>,
}

impl NpcGroup {
    pub const PLACEHOLDER: NpcGroupId =
        unsafe { NpcGroupId::new_unchecked(138296354938823594217663600u128) };
}
