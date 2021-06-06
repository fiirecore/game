use serde::Serialize;

use deps::{
	Identifiable,
	StaticRef,
	BorrowableMut,
};

use crate::{
	types::{PokemonType, Effective},
	pokemon::{
		Pokemon,
		PokemonId,
		PokemonRef,
		Level,
		Health,
		Gender,
		Experience,
		Friendship,
		status::StatusEffect,
		stat::{Stats, BaseStats},
	},
	moves::{
		Move,
		MoveRef,
		MoveCategory,
		instance::{
			MoveInstance,
			MoveInstanceSet,
		},
		persistent::PersistentMoveInstance,
	},
	item::ItemRef,
};

mod deserialize;

mod moves;
mod item;

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
	pub ivs: Stats,
    #[serde(default)]
    pub evs: Stats,

    #[serde(default)]
	pub experience: Experience,

    #[serde(default = "default_friendship")]
    pub friendship: Friendship,

	pub moves: MoveInstanceSet,

	#[serde(default)]
    pub status: Option<StatusEffect>,

	#[serde(default)]
	pub item: Option<ItemRef>,

	#[serde(skip)]
	pub persistent: Option<PersistentMoveInstance>, // to - do

	#[serde(skip)]
	pub base: BaseStats,

	pub current_hp: Health,
	
}

pub type BorrowedPokemon = BorrowableMut<'static, PokemonInstance>;

impl PokemonInstance {

	pub fn generate(id: PokemonId, min: Level, max: Level, ivs: Option<Stats>) -> Self {
		let pokemon = Pokemon::get(&id).value();

        let level = if min == max {
			max
		} else {
			crate::RANDOM.gen_range(min, max + 1)
		};

		let ivs = ivs.unwrap_or_else(Stats::random);
		let evs = Stats::default();

		let base = BaseStats::new(pokemon, &ivs, &evs, level);

		Self {

			nickname: None,
			level,
			gender: pokemon.generate_gender(),

			ivs,
			evs,

			experience: 0,
			friendship: 70,

			persistent: None,

			moves: pokemon.generate_moves(level),

			item: None,

			status: None,

			current_hp: base.hp(),

			base,
			
			pokemon: StaticRef::Init(pokemon),
			
		}
	}

	pub fn add_exp(&mut self, experience: super::Experience) -> Option<(Level, Option<Vec<MoveRef>>)> {

		// add exp to pokemon

		self.experience += experience * 5;

		// level the pokemon up if they reach a certain amount of exp (and then subtract the exp by the maximum for the previous level)

		let mut moves = Vec::new();
		let prev = self.level;

		let gr = self.pokemon.value().training.growth_rate;

		while self.experience > gr.max_exp(self.level) {
			self.experience -= gr.max_exp(self.level);

			self.level_up();

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

	pub fn level_up(&mut self) {
		self.level += 1;
		self.base = BaseStats::new(self.pokemon.value(), &self.ivs, &self.evs, self.level);
	}

	pub fn generate_with_level(id: PokemonId, level: Level, ivs: Option<Stats>) -> Self {
		Self::generate(id, level, level, ivs)
	}

	pub fn fainted(&self) -> bool {
		self.current_hp == 0
	}

	pub fn name(&self) -> &str {
		self.nickname.as_ref().unwrap_or(&self.pokemon.value().name)
		// match self.nickname.as_ref() {
		//     Some(name) => Cow::Borrowed(name),
		//     None => Cow::Owned(self.pokemon.value().name.to_ascii_uppercase()),
		// }
	}

	pub fn hp(&self) -> Health {
		self.current_hp
	}

	pub fn max_hp(&self) -> Health {
		self.base.hp()
	}

	pub fn heal(&mut self) {
		self.heal_hp();
		self.heal_pp();
	}

	pub fn heal_hp(&mut self) {
		self.current_hp = self.max_hp();
	}

	pub fn heal_pp(&mut self) {
		for pmove in self.moves.iter_mut() {
			pmove.restore();
		}
	}

	pub fn moves_at_level(&self) -> Vec<MoveRef> {
		let mut moves = Vec::new();
		for pokemon_move in &self.pokemon.value().moves {
			if pokemon_move.level == self.level {
				moves.push(Move::get(&pokemon_move.move_id))
			}
		}
		moves
	}

	pub fn effective(&self, pokemon_type: PokemonType, category: MoveCategory) -> Effective {
		let pokemon = self.pokemon.value();
		let primary = pokemon_type.effective(pokemon.primary_type, category);
		if let Some(secondary) = pokemon.secondary_type {
			primary * pokemon_type.effective(secondary, category)
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