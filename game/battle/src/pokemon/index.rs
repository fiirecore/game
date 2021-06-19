use serde::{Deserialize, Serialize};
use pokedex::moves::target::PlayerId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct PokemonIndex {
    pub team: PlayerId,
    pub index: usize,
}

pub type ActivePokemonIndex = PokemonIndex;
pub type PartyPokemonIndex = PokemonIndex;

impl core::fmt::Display for PokemonIndex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?} #{}", self.team, self.index)
    }
}