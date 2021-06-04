use pokedex::pokemon::instance::BorrowedPokemon;

mod option;
mod moves;
mod party;

mod view;

pub use option::*;
pub use moves::*;
pub use party::*;
pub use view::*;

#[derive(Default)]
pub struct ActivePokemon {

    pub pokemon: PokemonOption,
    pub queued_move: Option<BattleMove>,

}

impl ActivePokemon {

    pub fn new(index: usize, pokemon: BorrowedPokemon) -> Self {
        Self {
            pokemon: PokemonOption::Some(index, pokemon),
            queued_move: None
        }
    }

    pub fn dequeue(&mut self) {
        self.queued_move = None;
    }

}

impl core::fmt::Debug for ActivePokemon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Debug::fmt(&self.pokemon, f)
    }
}