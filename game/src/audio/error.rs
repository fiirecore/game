use std::error::Error;
use core::fmt::Display;

#[derive(Debug)]
pub enum AddAudioError {
    #[cfg(feature = "audio")]
    Uninitialized,
    #[cfg(feature = "audio")]
    LockError(Lockable),
    #[cfg(feature = "audio")]
    TetraError(deps::tetra::TetraError),
}

#[derive(Debug)]
pub enum Lockable {
    AudioManager,
    AudioMap,
    CurrentMusic,
}

#[derive(Debug)]
pub enum PlayAudioError {
    #[cfg(feature = "audio")]
    Uninitialized,
    #[cfg(feature = "audio")]
    Missing,
    #[cfg(feature = "audio")]
    LockError(Lockable),
    #[cfg(feature = "audio")]
    TetraError(deps::tetra::TetraError),
}

impl Error for AddAudioError {}

impl Display for AddAudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(feature = "audio")] {
            match self {
                Self::Uninitialized => write!(f, "Audio manager is uninitialized!"),
                Self::LockError(locked) => write!(f, "{:?} is locked!", locked),
                Self::TetraError(err) => err.fmt(f),
            }
        }
        #[cfg(not(feature = "audio"))] {
            write!(f, "Audio is disabled by feature.")
        }
    }
}

impl Error for PlayAudioError {}

impl Display for PlayAudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(feature = "audio")] {
            match self {
                Self::Uninitialized => write!(f, "Audio manager is uninitialized!"),
                Self::Missing => write!(f, "Could not find music with specified id!"),
                Self::LockError(locked) => write!(f, "{:?} is locked!", locked),
                Self::TetraError(err) => err.fmt(f),
            }
        }
        #[cfg(not(feature = "audio"))] {
            write!(f, "Audio is disabled by feature.")
        }
    }
}