use serde::{Deserialize, Serialize};

use crate::pokemon::Pokemon;
use crate::moves::PokemonMove;
use crate::item::Item;

use crate::moves::GamePokemonMove;

#[derive(Deserialize, Serialize)]
pub struct SerializedDex {
	pub pokemon: Vec<SerializedPokemon>,
	pub moves: Vec<SerializedMove>,
    pub items: Vec<SerializedItem>,
}

#[derive(Deserialize, Serialize)]
pub struct SerializedPokemon {

    pub pokemon: Pokemon,
    pub cry_ogg: Vec<u8>,
    pub front_png: Vec<u8>,
    pub back_png: Vec<u8>,
    pub icon_png: Vec<u8>,

}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SerializedMove {

    #[serde(rename = "move")]
	pub pokemon_move: PokemonMove,

    #[serde(default)]
    pub game_move: Option<GamePokemonMove>,
	
}

impl From<PokemonMove> for SerializedMove {
    fn from(pokemon_move: PokemonMove) -> Self {
        Self {
            pokemon_move,
            game_move: None,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SerializedItem {

    pub item: Item,

    pub texture: Vec<u8>,
}