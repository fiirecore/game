use std::error::Error;
use core::fmt::Display;

#[derive(Debug)]
pub enum AddAudioError {
    #[cfg(feature = "play")]
    Uninitialized,
    #[cfg(feature = "play")]
    NoManager,

    #[cfg(all(not(any(target_arch = "wasm32", target_os = "android")), feature = "play"))]
    SetupError(kira::manager::error::SetupError),

    #[cfg(all(not(any(target_arch = "wasm32", target_os = "android")), feature = "play"))]
    DecodeError(kira::sound::error::SoundFromFileError),
    // #[cfg(target_arch = "wasm32")]
    // DecodeError(Box<dyn core::fmt::Debug>),
    #[cfg(all(not(any(target_arch = "wasm32", target_os = "android")), feature = "play"))]
    ManagerAddError(kira::manager::error::AddSoundError),
}

#[derive(Debug)]
pub enum PlayAudioError {
    // #[cfg(feature = "play")]
    // Uninitialized,
    #[cfg(feature = "play")]
    Missing,
    #[cfg(feature = "play")]
    CurrentLocked,
    #[cfg(all(not(any(target_arch = "wasm32", target_os = "android")), feature = "play"))]
    CommandError(kira::CommandError),
    #[cfg(all(any(target_arch = "wasm32", target_os = "android"), feature = "play"))]
    PlayError(),
}

impl Error for AddAudioError {}

impl Display for AddAudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(feature = "play")] {
            match self {
                Self::Uninitialized => write!(f, "Audio manager is uninitialized!"),
                Self::NoManager => write!(f, "Audio manager could not be found. (You probably forgot to initialize it)"),
                #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
                Self::SetupError(err) => Display::fmt(err, f),
                #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
                Self::DecodeError(err) => Display::fmt(err, f),
                #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
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
                #[cfg(all(not(any(target_arch = "wasm32", target_os = "android")), feature = "play"))]
                PlayAudioError::CommandError(err) => Display::fmt(err, f),
                #[cfg(all(any(target_arch = "wasm32", target_os = "android"), feature = "play"))]
                PlayAudioError::PlayError() => write!(f, "Could not play audio with error"),
            }
        }
        #[cfg(not(feature = "play"))] {
            write!(f, "Audio is disabled by feature.")
        }
    }
}