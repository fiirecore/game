use deps::vec::ArrayVec;
use pokedex::{
    pokemon::{Health, Level, PokemonRef, instance::PokemonInstance, party::PokemonParty},
    moves::target::PlayerId,
};

use crate::message::{Active, PartyIndex};

pub trait BattlePartyView {

    fn id(&self) -> &PlayerId;

    fn name(&self) -> &str;

    fn active(&self, active: Active) -> Option<&dyn PokemonView>;

    fn active_mut(&mut self, active: Active) -> Option<&mut dyn PokemonView>;

    fn active_len(&self) -> usize;

    fn len(&self) -> usize;

    fn active_eq(&self, active: Active, index: Option<PartyIndex>) -> bool;

    fn index(&self, active: Active) -> Option<PartyIndex>;

    fn pokemon(&self, index: PartyIndex) -> Option<&dyn PokemonView>;

    fn add(&mut self, index: PartyIndex, unknown: UnknownPokemon);

    fn replace(&mut self, active: Active, new: Option<PartyIndex>);

    fn any_inactive(&self) -> bool;
    
    // fn update_hp(&mut self, active: usize, hp: f32);
}

pub trait PokemonView {

    fn pokemon(&self) -> PokemonRef;

    fn name(&self) -> &str;
    fn level(&self) -> Level;

    fn set_hp(&mut self, hp: f32);
    fn hp(&self) -> f32;

    fn fainted(&self) -> bool;

    fn instance(&self) -> Option<&PokemonInstance>;

    // fn instance_mut(&mut self) -> Option<&mut PokemonInstance>;

}

// impl core::fmt::Debug for dyn PokemonView {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Lv{} {}", self.level(), self.name())
//     }
// }

impl PokemonView for PokemonInstance {
    fn pokemon(&self) -> PokemonRef {
        self.pokemon
    }

    fn name(&self) -> &str {
        PokemonInstance::name(self)
    }

    fn level(&self) -> Level {
        self.level
    }

    fn set_hp(&mut self, hp: f32) {
        self.current_hp = (hp.max(0.0) * self.max_hp() as f32) as Health;
    }

    fn hp(&self) -> f32 {
        self.percent_hp()
    }

    fn fainted(&self) -> bool {
        self.fainted()
    }

    fn instance(&self) -> Option<&PokemonInstance> {
        Some(self)
    }

    // fn instance_mut(&mut self) -> Option<&mut PokemonInstance> {
    //     Some(self)
    // }

}

#[derive(Debug, Clone)]
pub struct BattlePartyKnown {
    pub id: PlayerId,
    pub name: String,
    pub active: ArrayVec<[Option<usize>; 3]>,
    pub pokemon: PokemonParty,
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

#[derive(Debug, Clone)]
pub struct UnknownPokemon {
    pub pokemon: PokemonRef,
    pub level: Level,
    pub hp: f32, // % of hp
    pub instance: Option<PokemonInstance>,
}

impl UnknownPokemon {

    pub fn new(pokemon: &PokemonInstance) -> Self {
        Self {
            pokemon: pokemon.pokemon,
            level: pokemon.level,
            hp: pokemon.percent_hp(),
            instance: None,
        }
    }

}

impl PokemonView for Option<UnknownPokemon> {

    fn pokemon(&self) -> PokemonRef {
        match self {
            Some(pokemon) => pokemon.pokemon,
            None => PokemonRef::Uninit(pokedex::pokemon::UNKNOWN_POKEMON),
        }
    }

    fn name(&self) -> &str {
        match self {
            Some(pokemon) => &pokemon.pokemon.value().name,
            None => "Unknown",
        }
    }

    fn level(&self) -> Level {
        match self {
            Some(pokemon) => pokemon.level,
            None => 0,
        }
    }

    fn set_hp(&mut self, hp: f32) {
        if let Some(pokemon) = self {
            pokemon.hp = hp.max(0.0);
        }
    }

    fn hp(&self) -> f32 {
        match self {
            Some(p) => p.hp,
            None => 0.0,
        }
    }

    fn fainted(&self) -> bool {
        match self {
            Some(p) => p.hp == 0.0,
            None => false,
        }
    }

    fn instance(&self) -> Option<&PokemonInstance> {
        self.as_ref().map(|p| p.instance.as_ref()).flatten()
    }

    // fn instance_mut(&mut self) -> Option<&mut PokemonInstance> {
    //     self.as_mut().map(|p| p.instance.as_mut()).flatten()
    // }

}