use serde::{Deserialize, Serialize};

use crate::map::MusicId;

pub type NpcGroupId = tinystr::TinyStr16;
pub type TrainerGroupId = NpcGroupId;

#[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(deny_unknown_fields)]
pub struct NpcGroup {
    pub message: MessageColor,
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

impl From<MessageColor> for [f32; 4] {
    fn from(message: MessageColor) -> Self {
        match message {
            MessageColor::Black => [20.0 / 255.0, 20.0 / 255.0, 20.0 / 255.0, 1.0],
            MessageColor::White => [240.0 / 255.0, 240.0 / 255.0, 240.0 / 255.0, 1.0],
            MessageColor::Red => [0.90, 0.16, 0.22, 1.0],
            MessageColor::Blue => [48.0 / 255.0, 80.0 / 255.0, 200.0 / 255.0, 1.0],
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TrainerGroup {
    pub prefix: String,
    pub music: Option<MusicId>,
}

impl NpcGroup {
    /// Placeholder Npc Group ID
    pub const PLACEHOLDER: NpcGroupId =
        unsafe { NpcGroupId::new_unchecked(138296354938823594217663600u128) };
}

impl TrainerGroup {
    pub const PLACEHOLDER: TrainerGroupId = NpcGroup::PLACEHOLDER;
}