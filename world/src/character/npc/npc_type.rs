use serde::{Deserialize, Serialize};
use tinystr::TinyStr16;

use crate::character::sprite::SpriteIndexes;

pub type NpcTypeId = TinyStr16;
pub type BadgeId = TinyStr16;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum MessageColor {
    Black,
    White,
    Red,
    Blue,
}
#[derive(Debug, Clone)]
pub struct NpcType {
    pub message: MessageColor,
    pub sprite: SpriteIndexes,
    pub trainer: Option<TrainerType>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TrainerType {
    pub name: String,
    pub music: Option<TinyStr16>,
}
