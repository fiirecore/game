use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SpriteIndices {
    pub up: [u8; 4],
    pub down: [u8; 4],
    pub side: [u8; 4],
}

impl SpriteIndices {
    pub const STILL: Self = Self {
        up: [1; 4],
        down: [0; 4],
        side: [2; 4],
    };

    pub const WALK: Self = Self {
        up: [1, 5, 1, 6],
        down: [0, 3, 0, 4],
        side: [2, 7, 2, 8],
    };
}
