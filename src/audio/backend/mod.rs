#[cfg(feature = "audio")]
mod tetra;

#[cfg(feature = "audio")]
pub use self::tetra::*;

#[cfg(not(feature = "audio"))]
mod dummy;

#[cfg(not(feature = "audio"))]
pub use dummy::*;