pub mod wild_pokemon_encounter;
pub mod wild_pokemon_table;
pub mod original_wild_pokemon_table;
pub mod random_wild_pokemon_table;

use self::wild_pokemon_table::WildPokemonTable;

pub struct WildEntry {

    pub tiles: Vec<u16>,
    pub table: Box<dyn WildPokemonTable>,

}