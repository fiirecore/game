#[cfg(feature = "random")]
pub mod random;

#[cfg(feature = "log")]
pub extern crate log;

#[cfg(feature = "borrow")]
pub mod borrow;

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

    #[deprecated(note = "use hashbrown")]
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

#[cfg(feature = "engine")]
pub extern crate tetra;

#[cfg(all(feature = "engine", feature = "hash"))]
use {
    std::{hash::Hash, fmt::Display},
    hash::HashMap,
    tetra::graphics::Texture,
};

#[cfg(all(feature = "engine", feature = "hash"))]
pub trait TextureManager {

    type Id: Eq + Hash + Display;

    // type Map = hash::HashMap<Self::Id, tetra::graphics::Texture>;

    fn map<'a>() -> &'a mut Option<HashMap<Self::Id, Texture>>;

    fn name() -> &'static str {
        let name = std::any::type_name::<Self>();
        name.split("::").last().unwrap_or(name)
    }

    fn set(map: HashMap<Self::Id, Texture>) {
        *Self::map() = Some(map);
    }

    fn get(id: &Self::Id) -> &Texture {
        Self::try_get(id).unwrap_or_else(|| panic!("Could not get texture from exture manager \"{}\" with id {}", Self::name(), id))
    }

    fn try_get(id: &Self::Id) -> Option<&Texture> {
        Self::map().as_ref().unwrap_or_else(|| panic!("Texture manager \"{}\" has not been initialized!", Self::name())).get(id)
    }

}