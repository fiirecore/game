use serde::{Deserialize, Serialize};

use crate::{
    character::{CharacterGroupId, CharacterState},
    map::MusicId,
    message::MessageColor,
};

pub type TrainerGroupId = CharacterGroupId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcGroup {
    pub message: MessageColor,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TrainerGroup {
    pub prefix: String,
    pub music: Option<MusicId>,
}

impl TrainerGroup {
    pub const PLACEHOLDER: TrainerGroupId = CharacterState::PLACEHOLDER;
}
