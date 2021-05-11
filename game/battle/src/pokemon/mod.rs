use game::{
    pokedex::pokemon::instance::PokemonInstance,
    battle::BattleTeam,
};

mod party;
mod renderer;
mod moves;

pub use party::*;
pub use renderer::*;
pub use moves::*;

use crate::gui::status::PokemonStatusGui;


pub struct ActivePokemon {

    pub pokemon: PokemonOption,
    pub queued_move: Option<BattleMove>,

    pub status: PokemonStatusGui,
    pub renderer: ActivePokemonRenderer,

    pub last_move: Option<(usize, usize)>, // previous cursor pos

}

#[derive(Clone)]
pub enum PokemonOption {
    Some(usize, PokemonInstance),
    None,
    ToReplace(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ActivePokemonIndex {
	pub team: BattleTeam,
	pub active: usize,
}

impl PokemonOption {
    pub fn as_ref(&self) -> Option<&PokemonInstance> {
        match self {
            PokemonOption::Some(_, instance) => Some(instance),
            // PokemonOption::Replace(_, instance, _) => Some(instance),
            _ => None,
        }
    }
    pub fn as_mut(&mut self) -> Option<&mut PokemonInstance> {
        match self {
            PokemonOption::Some(_, instance) => Some(instance),
            // PokemonOption::Replace(_, instance, _) => Some(instance),
            _ => None,
        }
    }

    pub fn take(&mut self) -> PokemonOption {
        std::mem::replace(self, Self::None)
    }

    pub fn replace(&mut self, new: usize) -> Option<(usize, PokemonInstance)> {
        if match self {
            PokemonOption::ToReplace(..) => false,
            _ => true,
        } {
            if let PokemonOption::Some(index, instance) = self.take() {
                *self = PokemonOption::ToReplace(new);
                return Some((index, instance));
            } else {
                *self = PokemonOption::ToReplace(new);
            }
        }
        None
    }

    pub fn is_some(&self) -> bool {
        match self {
            PokemonOption::None => false,
            _ => true,
        }
    }

}

impl ActivePokemon {

    pub fn update(&mut self) {
        self.queued_move = None;
        let pokemon = self.pokemon.as_ref();
        self.status.update_gui(pokemon, true);
        self.renderer.update_pokemon(pokemon);
    }

}