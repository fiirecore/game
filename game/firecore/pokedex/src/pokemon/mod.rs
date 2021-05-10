use deps::hash::HashMap;
use serde::{Deserialize, Serialize};
use deps::Random;
use crate::moves::MoveId;
use crate::moves::instance::{MoveInstance, MoveInstanceSet};
use data::breeding::Breeding;
use data::LearnableMove;
use data::PokedexData;
use data::StatSet;
use data::training::Training;
use data::Gender;

pub type Pokedex = HashMap<PokemonId, Pokemon>;

pub static mut POKEDEX: Option<Pokedex> = None;

pub fn pokedex() -> &'static Pokedex {
	unsafe { POKEDEX.as_ref().expect("Pokedex was not initialized!") }
}

pub mod data;

pub mod types;
pub mod status;

pub mod saved;
pub mod instance;

pub static POKEMON_RANDOM: Random = Random::new();

pub type PokemonId = u16;
pub type Level = u8;
pub type Stat = u8;
pub type Experience = u32;
pub type Friendship = u8;
pub type Health = u16;

pub type PokemonRef = &'static Pokemon;

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
		moves.into_iter().map(|id| crate::moves::movedex().get(&id)).flatten().map(|pokemon_move| MoveInstance {
		    pp: pokemon_move.pp,
		    pokemon_move,
		}).collect()
		// moves.reverse();
		// moves.truncate(4);

		// return moves.into();		
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
	
}