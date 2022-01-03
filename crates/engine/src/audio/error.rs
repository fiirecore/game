use core::fmt::Display;
use std::error::Error;

#[derive(Debug)]
pub enum PlayAudioError {
    Missing,
    // #[cfg(feature = "audio")]
    // TetraError(tetra::TetraError),
}

impl Error for PlayAudioError {}

impl Display for PlayAudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Missing => write!(f, "Could not find music with specified id!"),
            // Self::TetraError(err) => err.fmt(f),
        }
    }
}
