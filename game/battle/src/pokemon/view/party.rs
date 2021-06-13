use deps::vec::ArrayVec;
use pokedex::{
    pokemon::instance::PokemonInstance,
    moves::target::PlayerId,
};

use crate::message::{Active, PartyIndex};

use super::{BattlePartyView, PokemonView, UnknownPokemon};

#[derive(Debug, Clone)]
pub struct BattlePartyKnown {
    pub id: PlayerId,
    pub name: String,
    pub active: ArrayVec<[Option<usize>; 3]>,
    pub pokemon: ArrayVec<[PokemonInstance; 6]>,
}

impl Default for BattlePartyKnown {
    fn default() -> Self {
        Self {
            id: "default".parse().unwrap(),
            name: String::new(),
            active: Default::default(),
            pokemon: Default::default(),
        }
    }
}

impl BattlePartyView for BattlePartyKnown {

    fn id(&self) -> &PlayerId {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
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

#[derive(Clone)]
pub struct BattlePartyUnknown {
    pub id: PlayerId,
    pub name: String,
    pub active: ArrayVec<[Option<usize>; 3]>,
    pub pokemon: ArrayVec<[Option<UnknownPokemon>; 6]>,
}

impl Default for BattlePartyUnknown {
    fn default() -> Self {
        Self {
            id: "default".parse().unwrap(),
            name: String::default(),
            active: Default::default(),
            pokemon: Default::default(),
        }
    }
}

impl BattlePartyUnknown {
    pub fn add_instance(&mut self, index: PartyIndex, instance: PokemonInstance) {
        if let Some(pokemon) = self.pokemon.get_mut(index) {
            let pokemon = pokemon.get_or_insert(UnknownPokemon::new(&instance));
            pokemon.instance = Some(instance);
        }
    }
}

impl BattlePartyView for BattlePartyUnknown {

    fn id(&self) -> &PlayerId {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
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