pub extern crate firecore_base as engine;
pub use firecore_pokedex_engine_builder::pokedex;
// pub use battle::pokedex;

// #[deprecated(note = "add battle moves to battle-gui crate")]
// pub mod battle_move;

pub(crate) mod data;
pub mod gui;
pub mod texture;

/// Holds the string "cry"
pub const CRY_ID: tinystr::TinyStr8 = unsafe { tinystr::TinyStr8::from_bytes_unchecked(7959107u64.to_ne_bytes()) };

pub use data::PokedexClientData;
pub use firecore_pokedex_engine_builder::{trainer_group::TrainerGroupId, SerializedPokedexEngine};
