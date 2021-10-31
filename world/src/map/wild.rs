use firecore_pokedex::pokemon::{owned::SavedPokemon, Level, PokemonId};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::map::TileId;

// pub const DEFAULT_ENCOUNTER: u8 = 21;
pub const CHANCES: &[u8; 12] = &[20, 20, 10, 10, 10, 10, 5, 5, 4, 4, 1, 1];

// pub struct WildEntry {

//     grass: WildGrassEntry,

// }

#[derive(Serialize, Deserialize)]
pub struct WildEntry {
    pub tiles: Option<Vec<TileId>>,
    #[serde(default = "default_ratio")]
    pub ratio: u8,
    pub pokemon: [WildPokemon; 12],
}

#[derive(Serialize, Deserialize)]
pub struct WildPokemon {
    #[serde(rename = "pokemon_id")]
    pub id: PokemonId,

    #[serde(rename = "min_level")]
    pub min: Level,

    #[serde(rename = "max_level")]
    pub max: Level,
}

impl WildEntry {
    pub fn should_encounter(&self, random: &mut impl Rng) -> bool {
        random.gen_range(u8::MIN..u8::MAX) < self.ratio
    }

    pub fn generate(&self, random: &mut impl Rng) -> SavedPokemon {
        let pokemon = &self.pokemon[encounter_index(random)];
        let level = random.gen_range(pokemon.min..=pokemon.max);
        SavedPokemon::generate(random, pokemon.id, level, None, None)
        // match self.encounter {
        //     Some(encounter) => encounter[get_counter()].generate(),
        //     None => PokemonInstance::generate(
        //         super::WILD_RANDOM.gen_range(0..firecore_pokedex::pokedex().len() as u32) as PokemonId + 1,
        //         1,
        //         100,
        //         Some(StatSet::random()),
        //     ),
        // }
    }
}

fn encounter_index(random: &mut impl Rng) -> usize {
    let chance = random.gen_range(1..100);
    let mut chance_counter = 0;
    let mut counter = 0;
    while chance > chance_counter {
        chance_counter += CHANCES[counter];
        counter += 1;
    }
    counter - 1
}

const fn default_ratio() -> u8 {
    21
}
