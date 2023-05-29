pub extern crate firecore_pokedex as pokedex;
use hashbrown::HashMap;

pub type SerializedPokemon = (
    enum_map::EnumMap<pokedex::pokemon::PokemonTexture, Vec<u8>>,
    Vec<u8>,
);

pub type TrainerGroupId = tinystr::TinyStr16;

pub type PokemonOutput = HashMap<pokedex::pokemon::PokemonId, SerializedPokemon>;

pub type ItemOutput = HashMap<pokedex::item::ItemId, Vec<u8>>;

pub type TrainerGroupOutput = HashMap<TrainerGroupId, Vec<u8>>;
