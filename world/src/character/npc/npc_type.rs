use audio::music::MusicName;
use serde::{Deserialize, Serialize};
use text::TextColor;
use tinystr::TinyStr16;

use crate::character::sprite::SpriteIndexes;

pub type NpcTypeId = TinyStr16;
pub type BadgeId = TinyStr16;

#[derive(Debug)]
pub struct NpcType {
    pub text_color: TextColor,
    pub sprite: &'static SpriteIndexes,
    pub trainer: Option<TrainerType>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TrainerType {
    pub name: String,
    #[serde(default)]
    pub badge: Option<BadgeId>,
    pub music: Option<MusicName>,
}
