use serde::{Deserialize, Serialize};
use deps::{
	Random,
	hash::HashMap,
};
use crate::{
	Identifiable,
	moves::{
		MoveId,
		Move,
		instance::{MoveInstance, MoveInstanceSet}
	},
	pokemon::{
		data::{
			breeding::Breeding,
			LearnableMove,
			PokedexData,
			training::Training,
			Gender,
		},
		stat::StatSet,
	}
};

pub type Pokedex = HashMap<PokemonId, Pokemon>;

pub static mut POKEDEX: Option<Pokedex> = None;

pub fn pokedex_len() -> PokemonId {
	unsafe { POKEDEX.as_ref().map(|dex| dex.len()).unwrap_or_default() as PokemonId }
}

pub mod data;
pub mod stat;
pub mod types;
pub mod status;
pub mod instance;
pub mod party;

pub type PokemonId = u16;
pub type Level = u8;
pub type Experience = u32;
pub type Friendship = u8;
pub type Health = stat::BaseStat;

pub static POKEMON_RANDOM: Random = Random::new();

// pub type PokemonRef = &'static Pokemon;

#[derive(Serialize, Deserialize)]
pub struct Pokemon {

	pub data: PokedexData,
	pub base: StatSet,

	pub training: Training,
	pub breeding: Breeding,
	
	pub moves: Vec<LearnableMove>,
	
}

impl Pokemon {

	pub fn generate_moves(&self, level: Level) -> MoveInstanceSet {
		let mut moves = self.moves.iter().filter(|learnable_move| learnable_move.level <= level).map(|learnable_move| learnable_move.move_id).collect::<Vec<MoveId>>();
		moves.dedup();
		moves.reverse();
		moves.truncate(4);
		// MoveInstanceSet::Init(
			moves.into_iter().map(|id| Move::get(&id)).map(|move_ref| MoveInstance::new(move_ref)).collect()
		// )
	}

    pub fn generate_gender(&self) -> Gender {
        match self.breeding.gender {
            Some(percentage) => if POKEMON_RANDOM.gen_range(0, 8) > percentage {
                Gender::Male
            } else {
                Gender::Female
            }
            None => Gender::None,
        }
    }

	pub fn exp_from(&self, level: Level) -> Experience {
		((self.training.base_exp * level as u16) / 7) as Experience
	}
	
}

impl Identifiable for Pokemon {
    type Id = PokemonId;

    fn id(&self) -> &Self::Id {
        &self.data.id
    }

	fn try_get(id: &Self::Id) -> Option<&'static Self> where Self: Sized {
		unsafe { POKEDEX.as_ref().map(|map| map.get(id)).flatten() }
	}

}

pub type PokemonRef = crate::Ref<Pokemon>;

pub const fn default_iv() -> StatSet {
    StatSet::uniform(15)
}

pub const fn default_friendship() -> Friendship {
    70
}