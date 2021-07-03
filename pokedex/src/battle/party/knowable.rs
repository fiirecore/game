use crate::{
    battle::{view::UnknownPokemon, PartyIndex},
    pokemon::instance::{BorrowedPokemon, PokemonInstance},
};

use super::BattleParty;

pub type BattlePartyKnown<ID> = BattleParty<ID, Option<usize>, BorrowedPokemon>;
pub type BattlePartyUnknown<ID> = BattleParty<ID, Option<usize>, Option<UnknownPokemon>>;

impl<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + PartialEq> BattlePartyUnknown<ID> {
    pub fn add_instance(&mut self, index: PartyIndex, instance: PokemonInstance) {
        if let Some(pokemon) = self.pokemon.get_mut(index) {
            let pokemon = pokemon.get_or_insert(UnknownPokemon::new(&instance));
            pokemon.instance = Some(instance);
        }
    }

    pub fn add_unknown(&mut self, index: usize, unknown: UnknownPokemon) {
        self.pokemon[index] = Some(unknown);
    }
}
