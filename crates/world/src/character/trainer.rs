use serde::{Deserialize, Serialize};

use pokedex::{
    item::bag::SavedBag,
    pokemon::{owned::SavedPokemon, party::Party},
};

pub type Worth = u32;

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Trainer {
    pub party: Party<SavedPokemon>,
    pub bag: SavedBag,
    pub worth: Worth,
}
