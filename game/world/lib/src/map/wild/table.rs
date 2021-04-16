use firecore_pokedex::pokemon::{
    PokemonId,
    instance::PokemonInstance,
    data::StatSet,
    GeneratePokemon,
};

use super::GenerateWild;
use super::encounter::WildPokemonEncounter;

pub const DEFAULT_ENCOUNTER_CHANCE: u8 = 21;
pub const CHANCES: [usize; 12] = [20, 20, 10, 10, 10, 10, 5, 5, 4, 4, 1, 1];

#[derive(serde::Deserialize, serde::Serialize)]
pub struct WildPokemonTable {
    pub encounter_ratio: u8,
    pub encounter: Option<[WildPokemonEncounter; 12]>,
}

impl WildPokemonTable {

    pub fn try_encounter(&self) -> bool {
        (super::WILD_RANDOM.gen_range(0..255u32) as u8) < self.encounter_ratio
    }

}

impl GenerateWild for WildPokemonTable {

    fn generate(&self) -> PokemonInstance {
        match self.encounter {
            Some(encounter) => encounter[get_counter()].generate(),
            None => PokemonInstance::generate(
                super::WILD_RANDOM.gen_range(0..firecore_pokedex::pokedex().len() as u32) as PokemonId + 1, 
                1,
                100,
                Some(StatSet::random()),
            ),
        }
    }

}

impl Default for WildPokemonTable {
    fn default() -> Self {
        Self {
            encounter_ratio: DEFAULT_ENCOUNTER_CHANCE,
            encounter: None,
        }
    }
}

fn get_counter() -> usize {
    let chance = super::WILD_RANDOM.gen_range(1..100) as usize;
    let mut chance_counter = 0;
    let mut counter = 0;
    while chance > chance_counter {
        chance_counter += CHANCES[counter];
        counter+=1;            
    }
    counter - 1
}