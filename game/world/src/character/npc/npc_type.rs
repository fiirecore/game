use serde::{Deserialize, Serialize};
use deps::str::TinyStr16;
use firecore_font::message::TextColor;
use audio::music::MusicName;

use crate::character::sprite::SpriteIndexes;

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