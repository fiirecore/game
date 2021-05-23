#[cfg(all(not(any(target_arch = "wasm32", target_os = "android")), feature = "play"))]
mod kira;
// pub mod sdl;

#[cfg(all(not(any(target_arch = "wasm32", target_os = "android")), feature = "play"))]
pub use self::kira::*;

// #[cfg(target_arch = "wasm32")]
// pub mod quadsnd;