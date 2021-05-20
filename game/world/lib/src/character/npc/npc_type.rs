use serde::{Deserialize, Serialize};
use deps::tinystr::TinyStr16;
use firecore_font::message::TextColor;
use firecore_audio_lib::music::MusicName;

use crate::character::sprite::SpriteIndexes;

pub type NPCTypeId = TinyStr16;

#[derive(Debug)]
pub struct NPCType {

    pub text_color: TextColor,
    pub sprite: &'static SpriteIndexes,
    pub trainer: Option<TrainerType>,

}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TrainerType {

    pub name: String,
    #[serde(default)]
    pub gym_leader: bool,
    pub music: Option<MusicName>,

}