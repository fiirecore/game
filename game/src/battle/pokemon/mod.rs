use crate::pokedex::{
    pokemon::Level,
    moves::target::Team,
};

mod option;
mod party;
mod moves;

pub use option::*;
pub use party::*;
pub use moves::*;

use crate::battle::ui::pokemon::{
    PokemonRenderer,
    status::PokemonStatusGui,
};

pub struct ActivePokemon {

    pub pokemon: PokemonOption,
    pub queued_move: Option<BattleMove>,

    pub status: PokemonStatusGui,
    pub renderer: PokemonRenderer,

    pub last_move: Option<(usize, usize)>, // previous cursor pos

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ActivePokemonIndex {
	pub team: Team,
	pub active: usize,
}

impl ActivePokemon {

    pub fn reset(&mut self) {
        self.queued_move = None;
        self.update();
    }

    pub fn update(&mut self) {
        let pokemon = self.pokemon.as_ref();
        self.status.update_gui(pokemon.map(|i| (i.level, i)), true);
        self.renderer.new_pokemon(pokemon);
    }
    
    pub fn update_status(&mut self, level: Level, reset: bool) {
        self.status.update_gui(self.pokemon.as_ref().map(|i| (level, i)), reset)
    }

}

impl core::fmt::Debug for ActivePokemon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_struct("ActivePokemon")
        core::fmt::Debug::fmt(&self.pokemon, f)
    }
}

impl core::fmt::Display for ActivePokemonIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} #{}", self.team, self.active)
    }
}