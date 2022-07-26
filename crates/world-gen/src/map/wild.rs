use std::sync::Arc;

use hashbrown::HashMap;

use firecore_world_builder::world::{
    map::wild::{WildEntry, WildPokemon},
    pokedex::{pokemon::Pokemon, BasicDex},
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonWildEncounters {
    pub wild_encounter_groups: Vec<JsonWildEncounterGroup>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonWildEncounterGroup {
    pub label: String,
    pub for_maps: bool,
    pub fields: Vec<JsonWildType>,
    pub encounters: Vec<JsonWildEncounter>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonWildType {
    #[serde(rename = "type")]
    pub kind: String,
    pub encounter_rates: Vec<u8>,
    #[serde(default)]
    pub groups: HashMap<String, Vec<u8>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonWildEncounter {
    pub map: String,
    pub base_label: String,
    #[serde(default)]
    pub land_mons: Option<JsonWildEncounterType>,
    #[serde(default)]
    pub water_mons: Option<JsonWildEncounterType>,
    #[serde(default)]
    pub rock_smash_mons: Option<JsonWildEncounterType>,
    #[serde(default)]
    pub fishing_mons: Option<JsonWildEncounterType>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonWildEncounterType {
    pub encounter_rate: u8,
    pub mons: Vec<JsonWildPokemon>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonWildPokemon {
    pub min_level: u8,
    pub max_level: u8,
    pub species: String,
}

impl JsonWildEncounterType {
    pub fn into(self, pokedex: &BasicDex<Pokemon, Arc<Pokemon>>) -> WildEntry {
        WildEntry {
            ratio: self.encounter_rate,
            encounters: self
                .mons
                .into_par_iter()
                .flat_map(|mut p| {
                    let species = &mut p.species[8..];
                    unsafe {
                        let find = '_' as u8;
                        let replace = '-' as u8;
                        species.as_bytes_mut().iter_mut().for_each(|u| {
                            if *u == find {
                                *u = replace;
                            }
                        })
                    }
                    pokedex.try_get_named(&species)
                        .map(|species| WildPokemon {
                            species: species.id,
                            levels: p.min_level..=p.max_level,
                        })
                        .or_else(|| {
                            eprintln!(
                                "Could not get wild pokemon species {} because it does not exist!",
                                p.species
                            );
                            None
                        })
                })
                .collect(),
        }
    }

}
