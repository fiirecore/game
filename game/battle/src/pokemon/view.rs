use pokedex::{
    pokemon::{Level, PokemonRef, instance::PokemonInstance},
    moves::target::PlayerId,
};

use crate::message::{Active, PartyIndex};

mod pokemon;
mod party;

pub use pokemon::*;
pub use party::*;

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

    // fn pokemon_mut(&mut self, index: PartyIndex) -> Option<&mut dyn PokemonView>;

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

    fn instance_mut(&mut self) -> Option<&mut PokemonInstance>;

    // fn instance_mut(&mut self) -> Option<&mut PokemonInstance>;

}

// impl core::fmt::Debug for dyn PokemonView {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Lv{} {}", self.level(), self.name())
//     }
// }