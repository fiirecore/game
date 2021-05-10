use serde::{Deserialize, Serialize};
use deps::{
    hash::HashMap,
    tinystr::TinyStr8,
};

pub use pokedex::moves::*;

pub mod battle_script;

pub type FieldMoveId = TinyStr8;

pub type GameMoveDex = HashMap<MoveId, GamePokemonMove>;
pub static mut GAME_MOVE_DEX: Option<GameMoveDex> = None;

pub fn get_game_move(id: &MoveId) -> Option<GameMoveRef> {
    unsafe { GAME_MOVE_DEX.as_ref() }.map(|game_dex| game_dex.get(id)).flatten()
}

pub type GameMoveRef = &'static GamePokemonMove;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GamePokemonMove {

    pub field_id: Option<FieldMoveId>,    

}