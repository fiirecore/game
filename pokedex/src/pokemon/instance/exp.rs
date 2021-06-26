use std::ops::Range;

use crate::{
    moves::MoveRef,
    pokemon::{stat::BaseStats, Experience, Level},
};

use super::PokemonInstance;

impl PokemonInstance {
    pub fn add_exp(&mut self, experience: super::Experience) {
        // add exp to pokemon

        self.experience += experience * 5;

        // level the pokemon up if they reach a certain amount of exp (and then subtract the exp by the maximum for the previous level)

        let gr = self.pokemon.value().training.growth_rate;

        while self.experience > gr.max_exp(self.level) {
            self.experience -= gr.max_exp(self.level);

            self.level_up();

            // Get the moves the pokemon learns at the level it just gained.

            // Add moves if the player's pokemon does not have a full set of moves;

        }
    }

    pub fn moves_from(&self, levels: Range<Level>) -> Vec<MoveRef> {
        let levels = Range { start: levels.start + 1, end: levels.end + 1 };

        let mut moves = Vec::new();

        levels.for_each(|level| moves.extend(self.pokemon.value().moves_at_level(level)));

        moves        
    }

    pub fn exp_from(&self) -> Experience {
        self.pokemon.value().exp_from(self.level)
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        self.base = BaseStats::new(self.pokemon.value(), &self.ivs, &self.evs, self.level);
    }
}