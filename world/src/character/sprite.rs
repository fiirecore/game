use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct SpriteIndexes {
    pub up: [u8; 4],
    pub down: [u8; 4],
    pub side: [u8; 4],
}

#[derive(Deserialize, Serialize)]
pub enum SpriteIndexType {
    Still,
    Walk,
}

impl SpriteIndexes {
    pub const SPRITE_TYPE_STILL: SpriteIndexes = SpriteIndexes {
        up: [1; 4],
        down: [0; 4],
        side: [2; 4],
    };

    pub const SPRITE_TYPE_WALK: SpriteIndexes = SpriteIndexes {
        up: [1, 5, 1, 6],
        down: [0, 3, 0, 4],
        side: [2, 7, 2, 8],
    };

    pub const fn from_index(index: SpriteIndexType) -> &'static Self {
        match index {
            SpriteIndexType::Still => &Self::SPRITE_TYPE_STILL,
            SpriteIndexType::Walk => &Self::SPRITE_TYPE_WALK,
        }
    }
}
