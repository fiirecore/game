use hashbrown::HashMap;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;

use pokedex::pokemon::{owned::SavedPokemon, party::Party, Level, PokemonId};

use super::battle::BattleEntry;

pub type Ratio = u8;

pub type WildEntries = HashMap<WildType, WildEntry>;
pub type WildChances = HashMap<WildType, Vec<u8>>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WildType {
    Land,
    Rock,
    Water,
    Fishing(u8),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WildEntry {
    pub ratio: Ratio,
    pub encounters: Vec<WildPokemon>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WildPokemon {
    pub species: PokemonId,
    pub levels: RangeInclusive<Level>,
}

impl WildEntry {
    pub fn should_encounter(&self, random: &mut impl Rng) -> bool {
        random.gen_range(Ratio::MIN..Ratio::MAX) < self.ratio
    }

    pub fn generate(
        chances: &WildChances,
        t: &WildType,
        entry: &WildEntry,
        random: &mut impl Rng,
    ) -> Option<BattleEntry> {
        if entry.should_encounter(random) {
            let chances = match chances.get(t) {
                Some(chances) => chances,
                None => return None,
            };
            let pokemon = match entry.encounters.get(encounter_index(chances, random)) {
                Some(pokemon) => pokemon,
                None => return None,
            };
            let level = random.gen_range(pokemon.levels.clone());
            let pokemon = SavedPokemon::generate(pokemon.species, level, None, None);
            let mut party = Party::new();
            party.push(pokemon);
            return Some(BattleEntry {
                party,
                active: 1,
                trainer: None,
            });
        }
        None
    }
}

fn encounter_index(chances: &[u8], random: &mut impl Rng) -> usize {
    let mut chance = random.gen_range(1..100u8);
    let mut counter = 0;
    while chance > 0 {
        match chances.get(counter) {
            Some(by) => chance = chance.saturating_sub(*by),
            None => return counter.saturating_sub(1),
        }
        counter += 1;
    }
    counter - 1
}
