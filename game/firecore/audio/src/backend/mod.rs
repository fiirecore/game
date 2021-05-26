#[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
mod kira;
// mod soloud;

#[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
pub use self::kira::*;

// #[cfg(target_arch = "wasm32")]
// pub mod quadsnd;

#[cfg(any(target_arch = "wasm32", target_os = "android"))]
mod dummy;

#[cfg(any(target_arch = "wasm32", target_os = "android"))]
pub use dummy::*;

#[cfg(not(feature = "play"))]
mod dummy;

#[cfg(not(feature = "play"))]
pub use dummy::*;