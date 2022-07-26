pub extern crate firecore_base as engine;
pub extern crate firecore_pokedex as pokedex;
// pub use battle::pokedex;

// #[deprecated(note = "add battle moves to battle-gui crate")]
// pub mod battle_move;

pub(crate) mod data;
pub mod gui;
pub mod texture;

/// Holds the string "cry"
pub const CRY_ID: tinystr::TinyStr8 =
    unsafe { tinystr::TinyStr8::from_bytes_unchecked(7959107u64.to_ne_bytes()) };

pub use data::PokedexClientData;

pub type TrainerGroupId = tinystr::TinyStr16;

pub type SerializedPokemon = (enum_map::EnumMap<pokedex::pokemon::PokemonTexture, Vec<u8>>, Vec<u8>);

pub type PokemonOutput = engine::utils::HashMap<pokedex::pokemon::PokemonId, SerializedPokemon>;

pub type ItemOutput = engine::utils::HashMap<pokedex::item::ItemId, Vec<u8>>;

pub type TrainerGroupOutput = engine::utils::HashMap<TrainerGroupId, Vec<u8>>;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SerializedPokedexEngine {
    pub pokemon: PokemonOutput,
    pub items: ItemOutput,
    pub trainer_groups: TrainerGroupOutput,
}