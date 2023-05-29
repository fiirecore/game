use std::sync::Arc;

use pokengine::pokedex::{
    ailment::LiveAilment,
    moves::{Move, PP},
    pokemon::{owned::OwnedPokemon, Experience, Health, Level, Pokemon},
};

use battle::{
    party::{ActivePokemon, PlayerParty},
    pokemon::{remote::InitUnknownPokemon, PokemonView},
};

use crate::BattleTrainer;

type Active = usize;
type PartyIndex = usize;

pub trait PlayerView<ID> {
    fn id(&self) -> &ID;

    fn name(&self) -> &str;

    fn active(&self, active: Active) -> Option<&dyn GuiPokemonView>;

    fn active_mut(&mut self, active: Active) -> Option<&mut dyn GuiPokemonView>;

    fn active_eq(&self, active: Active, index: Option<PartyIndex>) -> bool;

    fn pokemon(&self, index: PartyIndex) -> Option<&dyn GuiPokemonView>;

    fn replace(&mut self, active: Active, new: Option<PartyIndex>);

    /// for target panel
    fn names(&self) -> Vec<Option<String>>;
}

impl<ID, A: ActivePokemon, P: GuiPokemonView> PlayerView<ID>
    for PlayerParty<ID, A, P, BattleTrainer>
{
    fn id(&self) -> &ID {
        &self.id
    }

    fn name(&self) -> &str {
        self.name.as_deref().unwrap_or("Unknown")
    }

    fn active(&self, active: usize) -> Option<&dyn GuiPokemonView> {
        PlayerParty::active(self, active).map(|p| p as _)
    }

    fn active_mut(&mut self, active: usize) -> Option<&mut dyn GuiPokemonView> {
        PlayerParty::active_mut(self, active).map(|p| p as _)
    }

    fn active_eq(&self, active: usize, index: Option<usize>) -> bool {
        self.active
            .get(active)
            .map(|i| i.as_ref().map(A::index) == index)
            .unwrap_or_default()
    }

    fn pokemon(&self, index: usize) -> Option<&dyn GuiPokemonView> {
        self.pokemon.get(index).map(|p| p as _)
    }

    fn replace(&mut self, active: usize, new: Option<usize>) {
        PlayerParty::replace(self, active, new)
    }

    fn names(&self) -> Vec<Option<String>> {
        self.active
            .iter()
            .map(|i| {
                i.as_ref()
                    .map(|a| self.pokemon.get(a.index()))
                    .flatten()
                    .map(|p| p.name().to_owned())
            })
            .collect()
    }
}

pub trait GuiPokemonView: BasePokemonView {
    fn base(&self) -> &dyn BasePokemonView;

    fn instance(&mut self) -> Option<&mut OwnedPokemon>;
}

pub trait BasePokemonView: PokemonView {
    fn pokemon(&self) -> &Arc<Pokemon>;

    fn name(&self) -> &str;

    fn set_level(&mut self, level: Level);
    fn level(&self) -> Level;

    fn set_hp(&mut self, hp: f32);
    fn percent_hp(&self) -> f32;

    fn set_ailment(&mut self, effect: Option<LiveAilment>);
    fn ailment(&mut self) -> Option<&mut LiveAilment>;

    fn set_exp(&mut self, experience: Experience);

    fn exp(&self) -> Experience;

    fn decrement_pp(&mut self, pokemon_move: &Move, pp: PP);
}

impl BasePokemonView for OwnedPokemon {
    fn pokemon(&self) -> &Arc<Pokemon> {
        &self.pokemon
    }

    fn name(&self) -> &str {
        OwnedPokemon::name(self)
    }

    fn set_level(&mut self, level: Level) {
        self.level = level;
    }

    fn level(&self) -> Level {
        self.level
    }

    fn set_hp(&mut self, hp: f32) {
        self.hp = (hp.max(0.0) * self.max_hp() as f32) as Health
    }

    fn percent_hp(&self) -> f32 {
        OwnedPokemon::percent_hp(self)
    }

    fn set_ailment(&mut self, ailment: Option<LiveAilment>) {
        self.ailment = ailment;
    }

    fn ailment(&mut self) -> Option<&mut LiveAilment> {
        self.ailment.as_mut()
    }

    fn set_exp(&mut self, experience: Experience) {
        self.experience = experience;
    }

    fn exp(&self) -> Experience {
        self.experience
    }

    fn decrement_pp(&mut self, pokemon_move: &Move, pp: PP) {
        if let Some(o) = self.moves.iter_mut().find(|o| o.0.id == pokemon_move.id) {
            o.1 -= pp;
        }
    }
}

impl GuiPokemonView for OwnedPokemon {
    fn base(&self) -> &dyn BasePokemonView {
        self
    }

    fn instance(&mut self) -> Option<&mut OwnedPokemon> {
        Some(self)
    }
}

impl BasePokemonView for Option<InitUnknownPokemon> {
    fn pokemon(&self) -> &Arc<Pokemon> {
        match self {
            Some(u) => &u.pokemon,
            None => todo!(),
        }
    }

    fn name(&self) -> &str {
        match self {
            Some(u) => u.name(),
            None => "Unknown",
        }
    }

    fn set_level(&mut self, level: Level) {
        if let Some(u) = self.as_mut() {
            u.level = level;
        }
    }

    fn level(&self) -> Level {
        self.as_ref().map(|u| u.level).unwrap_or_default()
    }

    fn set_hp(&mut self, hp: f32) {
        if let Some(u) = self.as_mut() {
            u.hp = hp;
        }
    }

    fn percent_hp(&self) -> f32 {
        self.as_ref().map(|v| v.hp).unwrap_or_default()
    }

    fn set_ailment(&mut self, ailment: Option<LiveAilment>) {
        if let Some(u) = self {
            u.ailment = ailment;
        }
    }

    fn ailment(&mut self) -> Option<&mut LiveAilment> {
        self.as_mut().map(|u| u.ailment.as_mut()).flatten()
    }

    fn set_exp(&mut self, _: Experience) {}

    fn exp(&self) -> Experience {
        0
    }

    fn decrement_pp(&mut self, _: &Move, _: PP) {}
}

impl GuiPokemonView for Option<InitUnknownPokemon> {
    fn base(&self) -> &dyn BasePokemonView {
        self
    }

    fn instance(&mut self) -> Option<&mut OwnedPokemon> {
        None
    }
}
