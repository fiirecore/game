use std::ops::Deref;

use crate::{
    battle::{view::UnknownPokemon, ActivePokemon},
    moves::MoveRef,
    pokemon::{
        instance::{BorrowedPokemon, PokemonInstance},
        party::{Party, PokemonParty},
    },
};

use super::{
    knowable::{BattlePartyKnown, BattlePartyUnknown},
    BattleParty,
};

#[derive(Debug, Clone)]
pub struct BattlePartyPokemon {
    pub pokemon: BorrowedPokemon,
    pub learnable_moves: Vec<MoveRef>,
    // pub persistent: Option<PersistentMove>,
    pub caught: bool,
    pub known: bool,
    pub flinch: bool,
    pub requestable: bool,
}

impl From<BorrowedPokemon> for BattlePartyPokemon {
    fn from(pokemon: BorrowedPokemon) -> Self {
        Self {
            pokemon,
            learnable_moves: Vec::new(),
            caught: false,
            known: false,
            flinch: false,
            requestable: false,
        }
    }
}

impl BattlePartyPokemon {
    pub fn know(&mut self) -> Option<UnknownPokemon> {
        (!self.known).then(|| {
            self.known = true;
            UnknownPokemon::new(&self.pokemon)
        })
    }
}

impl<ID, A, P> BattleParty<ID, A, P> {
    pub fn name(&self) -> &str {
        self.trainer
            .as_ref()
            .map(|t| t.name.as_str())
            .unwrap_or("Unknown")
    }
}

impl<ID> BattleParty<ID, ActivePokemon, BattlePartyPokemon> {
    pub fn all_fainted(&self) -> bool {
        !self
            .pokemon
            .iter()
            .any(|b| !b.pokemon.fainted() && !b.caught)
            || self.pokemon.is_empty()
    }

    pub fn any_inactive(&self) -> bool {
        self.pokemon
            .iter()
            .enumerate()
            .filter(|(i, _)| !self.active_contains(*i))
            .any(|(_, b)| !b.pokemon.fainted() && !b.caught)
    }

    pub fn active(&self, active: usize) -> Option<&BattlePartyPokemon> {
        self.active_index(active)
            .map(|index| self.pokemon.get(index))
            .flatten()
    }

    pub fn active_mut(&mut self, active: usize) -> Option<&mut BattlePartyPokemon> {
        self.active_index(active)
            .map(move |index| self.pokemon.get_mut(index))
            .flatten()
    }

    pub fn know(&mut self, index: usize) -> Option<UnknownPokemon> {
        self.pokemon.get_mut(index).map(|p| p.know()).flatten()
    }

    pub fn active_index(&self, index: usize) -> Option<usize> {
        self.active
            .get(index)
            .map(|active| active.index())
            .flatten()
    }

    pub fn active_contains(&self, index: usize) -> bool {
        self.active.iter().any(|active| match active {
            ActivePokemon::Some(i, _) => i == &index,
            _ => false,
        })
    }

    pub fn needs_replace(&self) -> bool {
        self.active
            .iter()
            .any(|a| matches!(a, ActivePokemon::ToReplace))
    }

    pub fn reveal_active(&mut self) {
        for active in self.active.iter() {
            if let Some(index) = active.index() {
                if let Some(pokemon) = self.pokemon.get_mut(index) {
                    pokemon.known = true;
                }
            }
        }
    }

    pub fn replace(&mut self, active: usize, new: Option<usize>) {
        self.active[active] = match new {
            Some(new) => ActivePokemon::Some(new, None),
            None => ActivePokemon::None,
        };
    }

    pub fn ready_to_move(&self) -> bool {
        self.active
            .iter()
            .filter(|a| a.is_active())
            .all(|a| match a {
                ActivePokemon::Some(_, m) => m.is_some(),
                _ => false,
            })
    }
}

impl<ID> BattleParty<ID, ActivePokemon, BattlePartyPokemon> {
    pub fn as_ref(&self) -> Party<&PokemonInstance> {
        self.pokemon.iter().map(|b| b.pokemon.deref()).collect()
    }
}

impl<ID> BattleParty<ID, ActivePokemon, BattlePartyPokemon> {
    pub fn cloned(&self) -> PokemonParty {
        self.pokemon.iter().map(|b| b.pokemon.deref().clone()).collect()
    }
}

impl<'a, ID: Copy> BattleParty<ID, ActivePokemon, BattlePartyPokemon> {
    pub fn as_known(&self) -> BattlePartyKnown<ID> {
        BattlePartyKnown {
            id: self.id,
            trainer: self.trainer.clone(),
            pokemon: self.pokemon.iter().map(|b| b.pokemon.deref().clone()).collect(),
            active: self.active.iter().map(|active| active.index()).collect(),
        }
    }

    pub fn as_unknown(&self) -> BattlePartyUnknown<ID> {
        BattlePartyUnknown {
            id: self.id,
            trainer: self.trainer.clone(),
            pokemon: self
                .pokemon
                .iter()
                .map(|p| p.known.then(|| UnknownPokemon::new(&p.pokemon)))
                .collect(),
            active: self.active.iter().map(|active| active.index()).collect(),
        }
    }
}