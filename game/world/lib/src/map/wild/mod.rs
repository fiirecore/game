use firecore_pokedex::pokemon::instance::PokemonInstance;
use deps::Random;
use serde::{Deserialize, Serialize};

use crate::TileId;

use self::table::WildPokemonTable;

pub mod encounter;
pub mod table;

pub static WILD_RANDOM: Random = Random::new();


#[derive(Serialize, Deserialize)]
pub struct WildEntry {

    pub tiles: Option<Vec<TileId>>,
    pub table: WildPokemonTable,

}

pub trait GenerateWild {

    fn generate(&self) -> PokemonInstance;

}