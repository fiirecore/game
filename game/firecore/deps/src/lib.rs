#[cfg(feature = "random")]
mod random;
#[cfg(feature = "random")]
pub use random::Random;

#[cfg(feature = "log")]
pub extern crate log;

#[cfg(feature = "engine")]
pub extern crate tetra;

#[cfg(feature = "borrow")]
mod borrow;

#[cfg(feature = "borrow")]
pub use borrow::*;

#[cfg(feature = "tinystr")]
pub extern crate tinystr as str;

#[cfg(feature = "vec")]

#[cfg(feature = "vec")]
pub mod vec {
    extern crate arrayvec;
    pub use arrayvec::ArrayVec as ArrayVec;
    // pub use arrayvec:: as arrayvec; // (macro)
}

#[deprecated(note = "use hashbrown")]
#[cfg(feature = "hash")]
pub mod hash {

    extern crate ahash;

	pub use ahash::{
        AHashMap as HashMap,
        AHashSet as HashSet,
        AHasher as Hasher,
    };
}

#[cfg(feature = "ser")]
extern crate bincode;

#[cfg(feature = "ser")]
pub mod ser {
    pub use {
        // postcard::to_allocvec as serialize,
        // postcard::from_bytes as deserialize,
        // postcard::Error,
        bincode::serialize,
        bincode::deserialize,
        bincode::Error,
        bincode::ErrorKind,
    };
}

#[cfg(feature = "script")]
pub extern crate rhai;