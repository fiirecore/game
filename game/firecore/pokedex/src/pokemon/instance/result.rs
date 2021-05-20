use crate::moves::Move;
use crate::pokemon::status::PokemonStatus;

pub struct MoveResult {
    pub move_ref: &'static Move,
    pub result: MoveResults,
}

#[derive(PartialEq, Eq)]
pub enum MoveResults {
    None,
    Miss,
    Damage,
    Status(PokemonStatus),
}