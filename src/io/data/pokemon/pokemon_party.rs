use serde::{Deserialize, Serialize};

use super::saved_pokemon::SavedPokemon;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PokemonParty {

	pub pokemon: Vec<SavedPokemon>,

}

impl PokemonParty {



}