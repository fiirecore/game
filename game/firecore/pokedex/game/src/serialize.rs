use serde::{Deserialize, Serialize};

use deps::hash::HashMap;

use pokedex::trainer::TrainerId;

use crate::pokemon::Pokemon;
use crate::moves::Move;
use crate::item::Item;

use crate::battle::serialized::SerializedBattleMoveBytes;

pub type SerializedTrainers = HashMap<TrainerId, Vec<u8>>;

#[derive(Deserialize, Serialize)]
pub struct SerializedDex {
	pub pokemon: Vec<SerializedPokemon>,
	pub moves: Vec<SerializedMove>,
    pub items: Vec<SerializedItem>,
    pub trainers: SerializedTrainers,
}

#[derive(Deserialize, Serialize)]
pub struct SerializedPokemon {

    pub pokemon: Pokemon,
    pub cry_ogg: Vec<u8>,
    pub front_png: Vec<u8>,
    pub back_png: Vec<u8>,
    pub icon_png: Vec<u8>,

}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SerializedMove {

    #[serde(rename = "move")]
	pub pokemon_move: Move,

    #[serde(default)]
    pub battle_move: Option<SerializedBattleMoveBytes>,
	
}

impl From<Move> for SerializedMove {
    fn from(pokemon_move: Move) -> Self {
        Self {
            pokemon_move,
            battle_move: None,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SerializedItem {
    pub item: Item,
    pub texture: Vec<u8>,
}