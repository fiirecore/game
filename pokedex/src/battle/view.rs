use deps::borrow::BorrowableMut;
use serde::{Deserialize, Serialize};

use crate::pokemon::{Health, Level, PokemonRef, instance::PokemonInstance};

use super::{party::BattleParty, Active, PartyIndex};

pub trait PokemonView {
    fn pokemon(&self) -> PokemonRef;

    fn name(&self) -> &str;

    fn set_level(&mut self, level: Level);
    fn level(&self) -> Level;

    fn set_hp(&mut self, hp: f32);
    fn hp(&self) -> f32;

    fn fainted(&self) -> bool;

    fn instance(&self) -> Option<&PokemonInstance>;

    fn instance_mut(&mut self) -> Option<&mut PokemonInstance>;

    // fn instance_mut(&mut self) -> Option<&mut PokemonInstance>;
}

pub trait BattlePartyView<ID> {
    fn id(&self) -> &ID;

    fn name(&self) -> &str;

    fn active(&self, active: Active) -> Option<&dyn PokemonView>;

    fn active_mut(&mut self, active: Active) -> Option<&mut dyn PokemonView>;

    fn active_len(&self) -> usize;

    fn len(&self) -> usize;

    fn active_eq(&self, active: Active, index: Option<PartyIndex>) -> bool;

    fn index(&self, active: Active) -> Option<PartyIndex>;

    fn pokemon(&self, index: PartyIndex) -> Option<&dyn PokemonView>;

    // fn pokemon_mut(&mut self, index: PartyIndex) -> Option<&mut dyn PokemonView>;

    fn replace(&mut self, active: Active, new: Option<PartyIndex>);

    fn any_inactive(&self) -> bool;

    // fn update_hp(&mut self, active: usize, hp: f32);
}

// impl core::fmt::Debug for dyn PokemonView {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Lv{} {}", self.level(), self.name())
//     }
// }

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct UnknownPokemon {
    pokemon: PokemonRef,
    name: String,
    level: Level,
    hp: f32, // % of hp
    pub instance: Option<PokemonInstance>,
}

impl UnknownPokemon {
    pub fn new(pokemon: &PokemonInstance) -> Self {
        Self {
            pokemon: pokemon.pokemon,
            name: pokemon.name().to_owned(),
            level: pokemon.level,
            hp: pokemon.percent_hp(),
            instance: None,
        }
    }
}

impl PokemonView for UnknownPokemon {
    fn pokemon(&self) -> PokemonRef {
        self.pokemon
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn level(&self) -> Level {
        self.level
    }

    fn set_level(&mut self, level: Level) {
        self.level = level;
    }

    fn set_hp(&mut self, hp: f32) {
        self.hp = hp.max(0.0);
    }

    fn hp(&self) -> f32 {
        self.hp
    }

    fn fainted(&self) -> bool {
        self.hp == 0.0
    }

    fn instance(&self) -> Option<&PokemonInstance> {
        self.instance.as_ref()
    }

    fn instance_mut(&mut self) -> Option<&mut PokemonInstance> {
        self.instance.as_mut()
    }
}

impl PokemonView for PokemonInstance {
    fn pokemon(&self) -> PokemonRef {
        self.pokemon
    }

    fn name(&self) -> &str {
        PokemonInstance::name(self)
    }

    fn set_level(&mut self, level: Level) {
        self.level = level;
    }

    fn level(&self) -> Level {
        self.level
    }

    fn set_hp(&mut self, hp: f32) {
        self.current_hp = (hp.max(0.0) * self.max_hp() as f32) as Health
    }

    fn hp(&self) -> f32 {
        self.percent_hp()
    }

    fn fainted(&self) -> bool {
        PokemonInstance::fainted(self)
    }

    fn instance(&self) -> Option<&PokemonInstance> {
        Some(self)
    }

    fn instance_mut(&mut self) -> Option<&mut PokemonInstance> {
        Some(self)
    }
}

impl PokemonView for Option<UnknownPokemon> {
    fn pokemon(&self) -> PokemonRef {
        self.as_ref().map(|v| v.pokemon()).unwrap_or_default()
    }

    fn name(&self) -> &str {
        self.as_ref().map(|v| v.name()).unwrap_or("Unknown")
    }

    fn set_level(&mut self, level: Level) {
        if let Some(v) = self.as_mut() {
            v.set_level(level)
        }
    }

    fn level(&self) -> Level {
        self.as_ref().map(|v| v.level()).unwrap_or_default()
    }

    fn set_hp(&mut self, hp: f32) {
        if let Some(v) = self.as_mut() {
            v.set_hp(hp);
        }
    }

    fn hp(&self) -> f32 {
        self.as_ref().map(|v| v.hp()).unwrap_or_default()
    }

    fn fainted(&self) -> bool {
        self.as_ref().map(|v| v.fainted()).unwrap_or_default()
    }

    fn instance(&self) -> Option<&PokemonInstance> {
        self.as_ref().map(|u| u.instance()).flatten()
    }

    fn instance_mut(&mut self) -> Option<&mut PokemonInstance> {
        self.as_mut().map(|u| u.instance_mut()).flatten()
    }
}

impl<'a> PokemonView for BorrowableMut<'a, PokemonInstance> {
    fn pokemon(&self) -> PokemonRef {
        self.pokemon
    }

    fn name(&self) -> &str {
        PokemonInstance::name(self)
    }

    fn set_level(&mut self, level: Level) {
        self.level = level;
    }

    fn level(&self) -> Level {
        self.level
    }

    fn set_hp(&mut self, hp: f32) {
        self.current_hp = (hp.max(0.0) * self.max_hp() as f32) as Health
    }

    fn hp(&self) -> f32 {
        self.percent_hp()
    }

    fn fainted(&self) -> bool {
        PokemonInstance::fainted(self)
    }

    fn instance(&self) -> Option<&PokemonInstance> {
        Some(self)
    }

    fn instance_mut(&mut self) -> Option<&mut PokemonInstance> {
        Some(self)
    }
}

impl<ID, P: PokemonView> BattlePartyView<ID> for BattleParty<ID, Option<usize>, P> {
    fn id(&self) -> &ID {
        &self.id
    }

    fn name(&self) -> &str {
        BattleParty::name(&self)
    }

    fn active(&self, active: usize) -> Option<&dyn PokemonView> {
        self.active
            .get(active)
            .copied()
            .flatten()
            .map(|active| &self.pokemon[active] as _)
    }

    fn active_mut(&mut self, active: usize) -> Option<&mut dyn PokemonView> {
        self.active
            .get(active)
            .copied()
            .flatten()
            .map(move |active| &mut self.pokemon[active] as _)
    }

    fn active_len(&self) -> usize {
        self.active.len()
    }

    fn len(&self) -> usize {
        self.pokemon.len()
    }

    fn active_eq(&self, active: usize, index: Option<usize>) -> bool {
        self.active
            .get(active)
            .map(|i| i == &index)
            .unwrap_or_default()
    }

    fn index(&self, active: Active) -> Option<PartyIndex> {
        self.active.get(active).copied().flatten()
    }

    fn pokemon(&self, index: usize) -> Option<&dyn PokemonView> {
        self.pokemon.get(index).map(|p| p as _)
    }

    fn replace(&mut self, active: usize, new: Option<usize>) {
        self.active[active] = new;
    }

    fn any_inactive(&self) -> bool {
        self.pokemon
            .iter()
            .enumerate()
            .any(|(i, p)| !(self.active.contains(&Some(i)) || p.fainted()))
    }
}
