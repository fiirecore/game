pub extern crate firecore_pokedex as pokedex;
use hashbrown::HashMap;

pub type SerializedPokemon = (
    enum_map::EnumMap<pokedex::pokemon::data::PokemonTexture, Vec<u8>>,
    Vec<u8>,
);

pub type PokemonOutput = HashMap<pokedex::pokemon::PokemonId, SerializedPokemon>;
pub type ItemOutput = HashMap<pokedex::item::ItemId, Vec<u8>>;
pub type TrainerGroupOutput = HashMap<pokedex::trainer::TrainerGroupId, Vec<u8>>;
