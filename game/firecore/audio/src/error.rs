use std::error::Error;
use core::fmt::Display;

#[derive(Debug)]
pub enum AddAudioError {
    #[cfg(feature = "play")]
    Uninitialized,
    #[cfg(feature = "play")]
    NoManager,
    
    #[cfg(feature = "sdl2")]
    SdlError(String),
    #[cfg(feature = "kira")]
    SetupError(kira::manager::error::SetupError),

    #[cfg(feature = "kira")]
    DecodeError(kira::sound::error::SoundFromFileError),
    #[cfg(feature = "kira")]
    ManagerAddError(kira::manager::error::AddSoundError),
}

#[cfg(feature = "sdl2")]
impl From<String> for AddAudioError {
    fn from(error: String) -> Self {
        Self::SdlError(error)
    }
}

#[derive(Debug)]
pub enum PlayAudioError {
    // #[cfg(feature = "play")]
    // Uninitialized,
    #[cfg(feature = "play")]
    Missing,
    #[cfg(feature = "play")]
    CurrentLocked,
    #[cfg(feature = "kira")]
    CommandError(kira::CommandError),
    #[cfg(feature = "sdl2")]
    SdlError(String),
    // #[cfg(all(any(target_arch = "wasm32", target_os = "android"), feature = "play"))]
    // PlayError(),
}

impl Error for AddAudioError {}

impl Display for AddAudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(feature = "play")] {
            match self {
                Self::Uninitialized => write!(f, "Audio manager is uninitialized!"),
                Self::NoManager => write!(f, "Audio manager could not be found. (You probably forgot to initialize it)"),
                #[cfg(feature = "sdl2")]
                Self::SdlError(err) => write!(f, "{}", err),
                #[cfg(feature = "kira")]
                Self::SetupError(err) => Display::fmt(err, f),
                #[cfg(feature = "kira")]
                Self::DecodeError(err) => Display::fmt(err, f),
                #[cfg(feature = "kira")]
                Self::ManagerAddError(err) => Display::fmt(err, f),
            }
        }
        #[cfg(not(feature = "play"))] {
            write!(f, "Audio is disabled by feature.")
        }
    }
}

impl Error for PlayAudioError {}

impl Display for PlayAudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(feature = "play")] {
            match self {
                // PlayAudioError::Uninitialized => write!(f, "Audio manager is uninitialized!"),
                PlayAudioError::Missing => write!(f, "Could not find music with specified id!"),
                PlayAudioError::CurrentLocked => write!(f, "Could not unlock current music mutex!"),
                #[cfg(feature = "sdl2")]
                Self::SdlError(err) => write!(f, "{}", err),
                #[cfg(feature = "kira")]
                PlayAudioError::CommandError(err) => Display::fmt(err, f),
            }
        }
        #[cfg(not(feature = "play"))] {
            write!(f, "Audio is disabled by feature.")
        }
    }
}

#[cfg(feature = "sdl2")]
impl From<String> for PlayAudioError {
    fn from(error: String) -> Self {
        Self::SdlError(error)
    }
}