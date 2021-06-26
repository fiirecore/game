use serde::Serialize;

use deps::borrow::{
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
		MoveRef,
		MoveCategory,
		instance::MoveInstanceSet,
		persistent::PersistentMoveInstance,
	},
	item::ItemRef,
};

mod deserialize;

mod moves;
mod item;
mod exp;

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
	pub persistent: Option<PersistentMoveInstance>,

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

	pub fn percent_hp(&self) -> f32 {
		self.hp() as f32 / self.max_hp() as f32
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
		self.pokemon.value().moves_at_level(self.level)
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