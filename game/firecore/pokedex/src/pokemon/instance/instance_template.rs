use serde::{Serialize, Deserialize};

use crate::{
	pokemon::{
		PokemonRef,
		Level,
		Health,
		Gender,
		Experience,
		Friendship,
		stat::{StatSet, BaseStatSet},
		status::StatusEffect,
		instance::Nickname,
		default_friendship,
		default_iv,
	},
	moves::instance::MoveInstanceSet,
	item::ItemRef,
};

use super::PokemonInstance;

#[derive(Clone, Deserialize, Serialize)]
struct PokemonInstance2 {
	
	#[serde(rename = "id")]
	pokemon: PokemonRef, 
	
	#[serde(default)]
    nickname: Nickname,
	
    level: Level,

	#[serde(default = "default_gender")]
    gender: Gender,
    
    #[serde(default = "default_iv")]
	ivs: StatSet,
    #[serde(default)]
    evs: StatSet,

    #[serde(default)]
	experience: Experience,

    #[serde(default = "default_friendship")]
    friendship: Friendship,

	// pub persistent: Option<PersistentMoveInstance>, // to - do

	#[serde(default = "default_moves")]
	moves: MoveInstanceSet,

	#[serde(default)]
    status: Option<StatusEffect>,

	#[serde(default)]
	item: Option<ItemRef>, // to - do

	#[serde(skip, default = "default_base")]
	base: BaseStatSet,

	#[serde(default = "default_current_hp")]
	current_hp: Health,
	
}

fn default_gender() -> Gender {
	Gender::None
}


fn default_moves() -> MoveInstanceSet {
	Default::default()
}

fn default_base() -> BaseStatSet {
	Default::default()
}

fn default_current_hp() -> Health {
	Default::default()
}

impl Into<PokemonInstance> for PokemonInstance2 {
    fn into(self) -> PokemonInstance {
        PokemonInstance {
            pokemon: self.pokemon,
            nickname: self.nickname,
            level: self.level,
            gender: self.gender,
            ivs: self.ivs,
            evs: self.evs,
            experience: self.experience,
            friendship: self.friendship,
            moves: self.moves,
            status: self.status,
            item: self.item,
            base: self.base,
            current_hp: self.current_hp,
		}
    }
}