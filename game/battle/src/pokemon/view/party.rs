use serde::{Deserialize, Serialize};
use deps::vec::ArrayVec;
use pokedex::{
    pokemon::instance::PokemonInstance,
    trainer::TrainerId,
    trainer::TrainerData,
};

use crate::{message::{Active, PartyIndex}};

use super::{BattlePartyView, PokemonView, UnknownPokemon};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BattlePartyKnowable<P> {
    pub id: TrainerId,
    pub trainer: Option<TrainerData>,
    pub active: ArrayVec<[Option<usize>; 3]>,
    pub pokemon: ArrayVec<[P; 6]>,

}


pub type BattlePartyKnown = BattlePartyKnowable<PokemonInstance>;

impl<P> Default for BattlePartyKnowable<P> {
    fn default() -> Self {
        Self {
            id: "default".parse().unwrap(),
            trainer: None,
            active: Default::default(),
            pokemon: Default::default(),
        }
    }
}

impl BattlePartyView for BattlePartyKnown {

    fn id(&self) -> &TrainerId {
        &self.id
    }

    fn name(&self) -> &str {
        self.trainer.as_ref().map(|t| t.name.as_str()).unwrap_or("Unknown")
    }

    fn active(&self, active: usize) -> Option<&dyn PokemonView> {
        self.active.get(active).copied().flatten().map(|index| self.pokemon.get(index)).flatten().map(|i| i as _)
    }

    fn active_mut(&mut self, active: usize) -> Option<&mut dyn PokemonView> {
        self.active.get(active).copied().flatten().map(move |index| self.pokemon.get_mut(index)).flatten().map(|i| i as _)
    }

    fn active_len(&self) -> usize {
        self.active.len()
    }

    fn len(&self) -> usize {
        self.pokemon.len()
    }

    fn active_eq(&self, active: usize, index: Option<usize>) -> bool {
        self.active.get(active).map(|i| i == &index).unwrap_or_default()
    }

    fn index(&self, active: Active) -> Option<PartyIndex> {
        self.active.get(active).copied().flatten()
    }

    fn pokemon(&self, index: usize) -> Option<&dyn PokemonView> {
        self.pokemon.get(index).map(|i| i as _)
    }

    fn add(&mut self, _: usize, _: UnknownPokemon) {
        
    }

    fn replace(&mut self, active: usize, new: Option<usize>) {
        self.active[active] = new;
    }

    fn any_inactive(&self) -> bool {
        self.pokemon.iter().enumerate().any(|(i, p)| !(self.active.contains(&Some(i)) || p.fainted()))
    }

}

pub type BattlePartyUnknown = BattlePartyKnowable<Option<UnknownPokemon>>;

impl BattlePartyUnknown {
    pub fn add_instance(&mut self, index: PartyIndex, instance: PokemonInstance) {
        if let Some(pokemon) = self.pokemon.get_mut(index) {
            let pokemon = pokemon.get_or_insert(UnknownPokemon::new(&instance));
            pokemon.instance = Some(instance);
        }
    }
}

impl BattlePartyView for BattlePartyUnknown {

    fn id(&self) -> &TrainerId {
        &self.id
    }

    fn name(&self) -> &str {
        self.trainer.as_ref().map(|t| t.name.as_str()).unwrap_or("Unknown")
    }

    fn active(&self, active: usize) -> Option<&dyn PokemonView> {
        self.active.get(active).copied().flatten().map(|active| &self.pokemon[active] as _)
    }
    
    fn active_mut(&mut self, active: usize) -> Option<&mut dyn PokemonView> {
        // if let Some(active) = self.active.g
        self.active.get(active).copied().flatten().map(move |active| &mut self.pokemon[active] as _)
    }

    fn active_len(&self) -> usize {
        self.active.len()
    }

    fn len(&self) -> usize {
        self.pokemon.len()
    }

    fn active_eq(&self, active: usize, index: Option<usize>) -> bool {
        self.active.get(active).map(|i| i == &index).unwrap_or_default()
    }

    fn index(&self, active: Active) -> Option<PartyIndex> {
        self.active.get(active).copied().flatten()
    }

    fn pokemon(&self, index: usize) -> Option<&dyn PokemonView> {
        self.pokemon.get(index).map(|p| p as _)
    }

    fn add(&mut self, index: usize, unknown: UnknownPokemon) {
        self.pokemon[index] = Some(unknown);
    }

    fn replace(&mut self, active: usize, new: Option<usize>) {
        self.active[active] = new;
    }

    fn any_inactive(&self) -> bool {
        self.pokemon.iter().enumerate().filter(|(i, _)| !self.active.contains(&Some(*i))).any(|(_, unknown)| unknown.is_none() || !unknown.fainted())
    }

}