#[cfg(feature = "random")]
mod random;
#[cfg(feature = "random")]
pub use random::Random;

#[cfg(feature = "tinystr")]
pub extern crate tinystr;

#[cfg(feature = "vec")]
extern crate arrayvec;

#[cfg(feature = "vec")]
pub mod vec {
    pub use arrayvec::ArrayVec as ArrayVec;
    // pub use arrayvec:: as arrayvec; // (macro)
}

#[cfg(feature = "hash")]
extern crate ahash;

#[cfg(feature = "hash")]
pub mod hash {
	pub use ahash::{
        AHashMap as HashMap,
        AHashSet as HashSet,
        AHasher as Hasher,
    };
}