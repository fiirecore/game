use core::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MusicId(pub MusicNameId);

pub type MusicNameId = tinystr::TinyAsciiStr<16>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SoundId(pub SoundNameId, #[serde(default)] pub SoundVariant);

pub type SoundNameId = tinystr::TinyAsciiStr<8>;

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize, Serialize,
)]
pub enum SoundVariant {
    #[default]
    None,
    Num(u32),
    Str(tinystr::TinyStr4),
}

impl Display for MusicId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.0.fmt(f)
    }
}

impl Display for SoundId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.1 {
            SoundVariant::None => self.0.fmt(f),
            SoundVariant::Num(num) => write!(f, "{} #{num}", self.0),
            SoundVariant::Str(str) => write!(f, "{}: {str}", self.0),
        }
    }
}
