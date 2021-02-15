pub mod wild_pokemon_encounter;
pub mod wild_pokemon_table;

use self::wild_pokemon_table::WildPokemonTable;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct WildEntry {

    pub tiles: Option<Vec<u16>>,
    pub table: WildPokemonTable,

}