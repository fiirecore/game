use serde::{Deserialize, Serialize};

use crate::pokemon::Pokemon;
use crate::moves::PokemonMove;
use crate::item::Item;

#[derive(Deserialize, Serialize)]
pub struct SerializedDex {
	pub pokemon: Vec<SerializedPokemon>,
	pub moves: Vec<PokemonMove>,
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

// #[derive(Deserialize, Serialize)]
// pub struct SerializedMove {
//     pub pokemon_move: PokemonMove,
//     pub action_script: Option<crate::moves::battle_script::BattleActionScript>,
// }

// #[derive(Deserialize, Serialize)]
// pub struct SerializedPokemonMove {

//     pub pokemon_move: PokemonMove, // add move scripting stuff later

// }

#[derive(Deserialize, Serialize)]
pub struct SerializedItem {

    pub item: Item,

    pub texture: Vec<u8>,
}