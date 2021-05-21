use std::borrow::Cow;
use serde::Serialize;

use crate::{
	Identifiable,
	BorrowableMut,
	pokemon::{
		Pokemon,
		PokemonId,
		PokemonRef,
		Level,
		Health,
		Gender,
		Experience,
		Friendship,
		types::{PokemonType, Effective},
		status::StatusEffect,
		stat::{StatSet, BaseStatSet},
	},
	moves::{
		MoveRef,
		instance::{
			MoveInstance,
			MoveInstanceSet,
		},
		// persistent::PersistentMoveInstance,
	},
	item::ItemRef,
};

mod deserialize;

mod moves;
mod item;
mod result;

pub use result::*;

// pub mod instance_template;

pub type Nickname = Option<String>;

#[derive(Clone, Serialize)]
pub struct PokemonInstance {
	
	#[serde(rename = "id")]
	pub pokemon: PokemonRef, 
	
	#[serde(default)]
    pub nickname: Nickname,
    pub level: Level,
    pub gender: Gender,
    
    #[serde(default = "default_iv")]
	pub ivs: StatSet,
    #[serde(default)]
    pub evs: StatSet,

    #[serde(default)]
	pub experience: Experience,

    #[serde(default = "default_friendship")]
    pub friendship: Friendship,

	// pub persistent: Option<PersistentMoveInstance>, // to - do

	pub moves: MoveInstanceSet,

	#[serde(default)]
    pub status: Option<StatusEffect>,

	#[serde(default)]
	pub item: Option<ItemRef>, // to - do

	#[serde(skip)]
	pub base: BaseStatSet,

	pub current_hp: Health,
	
}

pub type BorrowedPokemon = BorrowableMut<'static, PokemonInstance>;

impl PokemonInstance {

	pub fn generate(id: PokemonId, min: Level, max: Level, ivs: Option<StatSet>) -> Self {
		let pokemon = Pokemon::get(&id).value();

        let level = if min == max {
			max
		} else {
			super::POKEMON_RANDOM.gen_range(min, max + 1) as u8
		};

		let ivs = ivs.unwrap_or(StatSet::random());
		let evs = StatSet::default();

		let base = BaseStatSet::get(pokemon, &ivs, &evs, level);

		Self {

			nickname: None,
			level: level,
			gender: pokemon.generate_gender(),

			ivs: ivs,
			evs: evs,

			experience: 0,
			friendship: 70,

			// persistent: None,

			moves: pokemon.generate_moves(level),

			item: None,

			status: None,

			current_hp: base.hp,

			base,
			
			pokemon: crate::Ref::Init(pokemon),
			
		}
	}

	pub fn add_exp(&mut self, experience: super::Experience) -> Option<(Level, Option<Vec<MoveRef>>)> {

		// add exp to pokemon

		self.experience += experience * 5;

		// level the pokemon up if they reach a certain amount of exp (and then subtract the exp by the maximum for the previous level)

		let mut moves = Vec::new();
		let prev = self.level;

		while self.experience > self.pokemon.value().training.growth_rate.max_exp(self.level) {
			self.experience -= self.pokemon.value().training.growth_rate.max_exp(self.level);
			self.level += 1;

			self.on_level_up();

			// Get the moves the pokemon learns at the level it just gained.

			moves.extend(self.moves_at_level());

			// Add moves if the player's pokemon does not have a full set of moves;

			if !self.moves.is_full() {
				while let Some(pmove) = moves.pop() {
					if !self.moves.is_full() {
						self.moves.push(MoveInstance::new(pmove));
					} else {
						break;
					}
				}
			}
		}
			
		if prev != self.level {
			Some((
				self.level,
				if !moves.is_empty() {
					Some(moves)
				} else {
					None
				}
			))
		} else {
			None
		}
	}

	pub fn on_level_up(&mut self) {
		self.base = BaseStatSet::get(self.pokemon.value(), &self.ivs, &self.evs, self.level);
	}

	pub fn generate_with_level(id: PokemonId, level: Level, ivs: Option<StatSet>) -> Self {
		Self::generate(id, level, level, ivs)
	}

	pub fn fainted(&self) -> bool {
		self.current_hp == 0
	}

	pub fn name(&self) -> Cow<'_, str> {
		match self.nickname.as_ref() {
		    Some(name) => Cow::Borrowed(name),
		    None => Cow::Owned(self.pokemon.value().data.name.to_ascii_uppercase()),
		}
	}

	pub fn moves_at_level(&self) -> Vec<MoveRef> {
		let mut moves = Vec::new();
		for pokemon_move in &self.pokemon.value().moves {
			if pokemon_move.level == self.level {
				moves.push(<crate::moves::Move as crate::Identifiable>::get(&pokemon_move.move_id))
			}
		}
		moves
	}

	pub fn effective(&self, pokemon_type: PokemonType) -> Effective {
		let primary = pokemon_type.effective(self.pokemon.value().data.primary_type);
		if let Some(secondary) = self.pokemon.value().data.secondary_type {
			primary * pokemon_type.effective(secondary)
		} else {
			primary
		}
	}
	
}

impl core::fmt::Debug for PokemonInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Display::fmt(&self, f)
    }
}

impl core::fmt::Display for PokemonInstance {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Lv. {} {}", self.level, self.name())
	}
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct OwnedPokemon {

//     pub original_trainer: String,
//     pub original_location: (String, Level),

// }