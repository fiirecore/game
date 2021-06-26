use crate::{
    pokemon::instance::PokemonInstance,
    moves::target::MoveTargetInstance, 
};

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
// pub struct ActivePokemonIndex {
// 	pub team: Team,
// 	pub active: usize,
// }

#[derive(Clone, Copy)]
pub struct PokemonTarget<'a> {
    pub pokemon: &'a PokemonInstance,
    pub active: MoveTargetInstance,
}