use firecore_pokedex::pokemon::{
    PokemonId,
    Level,
    instance::PokemonInstance,
    data::StatSet,
    GeneratePokemon,
};

#[derive(Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct WildPokemonEncounter {

    #[serde(rename = "pokemon_id")]
    pub pokemon: PokemonId,

    #[serde(rename = "min_level")]
    pub min: Level,

    #[serde(rename = "max_level")]
    pub max: Level,

}

impl super::GenerateWild for WildPokemonEncounter {
    fn generate(&self) -> PokemonInstance {
        PokemonInstance::generate(self.pokemon, self.min, self.max, Some(StatSet::random()))
    }
}