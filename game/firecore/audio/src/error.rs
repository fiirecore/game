use std::error::Error;
use core::fmt::Display;

#[derive(Debug)]
pub enum AddAudioError {
    #[cfg(feature = "play")]
    Uninitialized,
    #[cfg(feature = "play")]
    LockError(Lockable),

    #[cfg(feature = "soloud")]
    SoloudError(soloud::SoloudError),
    
    #[cfg(feature = "sdl2")]
    SdlError(String),

    #[cfg(all(not(any(target_arch = "wasm32", target_os = "android")), feature = "kira"))]
    SetupError(kira::manager::error::SetupError),
    #[cfg(all(not(any(target_arch = "wasm32", target_os = "android")), feature = "kira"))]
    DecodeError(kira::sound::error::SoundFromFileError),
    #[cfg(all(not(any(target_arch = "wasm32", target_os = "android")), feature = "kira"))]
    ManagerAddError(kira::manager::error::AddSoundError),
}

#[derive(Debug)]
pub enum Lockable {
    AudioManager,
    AudioMap,
    CurrentMusic,
}

#[cfg(feature = "soloud")]
impl From<soloud::SoloudError> for AddAudioError {
    fn from(error: soloud::SoloudError) -> Self {
        Self::SoloudError(error)
    }
}

#[cfg(feature = "sdl2")]
impl From<String> for AddAudioError {
    fn from(error: String) -> Self {
        Self::SdlError(error)
    }
}

#[derive(Debug)]
pub enum PlayAudioError {
    #[cfg(feature = "play")]
    Uninitialized,
    #[cfg(feature = "play")]
    Missing,
    #[cfg(feature = "play")]
    LockError(Lockable),
    #[cfg(all(not(any(target_arch = "wasm32", target_os = "android")), feature = "kira"))]
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
                Self::LockError(locked) => write!(f, "{:?} is locked!", locked),
                #[cfg(feature = "soloud")]
                Self::SoloudError(err) => err.fmt(f),
                #[cfg(feature = "sdl2")]
                Self::SdlError(err) => write!(f, "{}", err),
                #[cfg(all(not(any(target_arch = "wasm32", target_os = "android")), feature = "kira"))]
                Self::SetupError(err) => Display::fmt(err, f),
                #[cfg(all(not(any(target_arch = "wasm32", target_os = "android")), feature = "kira"))]
                Self::DecodeError(err) => Display::fmt(err, f),
                #[cfg(all(not(any(target_arch = "wasm32", target_os = "android")), feature = "kira"))]
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
                PlayAudioError::Uninitialized => write!(f, "Audio manager is uninitialized!"),
                PlayAudioError::Missing => write!(f, "Could not find music with specified id!"),
                PlayAudioError::LockError(locked) => write!(f, "{:?} is locked!", locked),
                #[cfg(feature = "sdl2")]
                Self::SdlError(err) => write!(f, "{}", err),
                #[cfg(all(not(any(target_arch = "wasm32", target_os = "android")), feature = "kira"))]
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