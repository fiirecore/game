use serde::{Deserialize, Serialize};

pub type MusicId = tinystr::TinyStr16;
pub type SoundId = tinystr::TinyStr8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize, Serialize)]
pub enum SoundVariant {
    None,
    Num(u32),
    Str(tinystr::TinyStr4),
}

impl Default for SoundVariant {
    fn default() -> Self {
        Self::None
    }
}
