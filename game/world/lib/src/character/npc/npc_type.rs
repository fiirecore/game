use serde::{Deserialize, Serialize};

use firecore_audio_lib::music::MusicName;
use firecore_util::battle::BattleType;

use crate::character::sprite::SpriteIndexes;

#[derive(Debug)]
pub struct NPCType {

    pub sprite: &'static SpriteIndexes,
    pub trainer: Option<TrainerType>,

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrainerType {

    pub name: String,
    #[serde(rename = "type")]
    pub battle_type: BattleType,
    pub music: Option<MusicName>,

}