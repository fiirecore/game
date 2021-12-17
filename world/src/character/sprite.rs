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
    pub const STILL: SpriteIndexes = SpriteIndexes {
        up: [1; 4],
        down: [0; 4],
        side: [2; 4],
    };

    pub const WALK: SpriteIndexes = SpriteIndexes {
        up: [1, 5, 1, 6],
        down: [0, 3, 0, 4],
        side: [2, 7, 2, 8],
    };
}
