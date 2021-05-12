use game::battle::BattleTeam;

mod option;
mod party;
mod moves;

pub mod ai;

pub use option::*;
pub use party::*;
pub use moves::*;

use crate::ui::pokemon::{
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
	pub team: BattleTeam,
	pub active: usize,
}


impl ActivePokemon {

    pub fn reset(&mut self) {
        self.queued_move = None;
        let pokemon = self.pokemon.as_ref();
        self.status.update_gui(pokemon, true);
        self.renderer.new_pokemon(pokemon);
    }

    pub fn update_status(&mut self, reset: bool) {
        self.status.update_gui(self.pokemon.as_ref(), reset)
    }

}