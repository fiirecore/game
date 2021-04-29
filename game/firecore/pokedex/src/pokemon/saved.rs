use deps::smallvec::SmallVec;
use serde::{Deserialize, Serialize};

use super::Health;
use super::status::StatusEffect;
use super::{Level, PokemonId, Experience, Friendship, data::Gender, data::StatSet};

use crate::item::ItemId;
use crate::moves::saved::SavedMoveSet;

pub type SavedPokemonParty = SmallVec<[SavedPokemon; 6]>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedPokemon {

    pub id: PokemonId,

    pub data: PokemonData,


    pub item: Option<ItemId>,
    pub moves: Option<SavedMoveSet>,
    pub current_hp: Option<Health>,
    pub owned_data: Option<OwnedPokemon>,
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonData {

    pub nickname: Option<String>,
    pub level: Level,
    pub gender: Gender,

    // #[serde(default)]
    // pub ability: Option<Ability>,
    pub status: Option<StatusEffect>,
    
    #[serde(default = "default_iv")]
	pub ivs: StatSet,
    #[serde(default)]
    pub evs: StatSet,

    #[serde(default)]
	pub experience: Experience,

    #[serde(default = "default_friendship")]
    pub friendship: Friendship,

    // #[serde(default)]
    // pub item: Option<Item>, // item: struct with name, texture, description, and singular script-like enum which activates function of item

    // #[serde(default)]

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnedPokemon {

    pub original_trainer: String,
    pub original_location: (String, Level),

}

pub const fn default_iv() -> StatSet {
    StatSet::uniform(15)
}

pub const fn default_friendship() -> Friendship {
    70
}