use serde::{Deserialize, Serialize};
use deps::{
	Identifiable,
	StaticRef,
};
use crate::{
	types::PokemonType,
	pokemon::{
		data::{
			breeding::Breeding,
			LearnableMove,
			PokedexData,
			training::Training,
			Gender,
		},
		stat::Stats,
	},
	moves::{
		MoveId,
		Move,
		instance::{MoveInstance, MoveInstanceSet}
	},
};

pub mod dex;
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

// pub type PokemonRef = &'static Pokemon;

#[derive(Serialize, Deserialize)]
pub struct Pokemon {

	pub id: PokemonId,
	pub name: String,

	pub primary_type: PokemonType,
	pub secondary_type: Option<PokemonType>,
	
	pub base: Stats,

	pub data: PokedexData,

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
            Some(percentage) => if crate::RANDOM.gen_range(0, 8) > percentage {
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
        &self.id
    }

	fn try_get(id: &Self::Id) -> Option<&'static Self> where Self: Sized {
		unsafe { dex::POKEDEX.as_ref().map(|map| map.get(id)).flatten() }
	}

}

pub type PokemonRef = StaticRef<Pokemon>;

pub fn default_iv() -> Stats {
    Stats::uniform(15)
}

pub const fn default_friendship() -> Friendship {
    70
}