use serde::{Deserialize, Serialize};
use deps::Random;
use crate::moves::instance::{MoveInstance, MoveInstanceSet};
use data::breeding::Breeding;
use data::LearnableMove;
use data::PokedexData;
use data::StatSet;
use data::training::Training;
use data::Gender;

pub mod data;

pub mod types;
pub mod status;

pub mod saved;
pub mod instance;

pub mod texture;

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

pub trait GeneratePokemon {

    fn generate(id: PokemonId, min: Level, max: Level, ivs: Option<StatSet>) -> Self;

    fn generate_with_level(id: PokemonId, level: Level, ivs: Option<StatSet>) -> Self where Self: Sized {
        GeneratePokemon::generate(id, level, level, ivs)
    }

}

impl Pokemon {

	pub fn moves_from_level(&self, level: u8) -> MoveInstanceSet {
		let mut moves: Vec<MoveInstance> = Vec::new();
		for learnable_move in &self.moves {
			if learnable_move.level <= level {
				if let Some(pokemon_move) =  crate::movedex().get(&learnable_move.move_id) {
					let mut has = false;
					for pmove in &moves {
						if pmove.pokemon_move.id == pokemon_move.id {
							has = true;
						}
					}
					if !has {
						moves.push(MoveInstance {
							pp: pokemon_move.pp,
							pokemon_move: pokemon_move,
						});
					}
					
				}
			}
		}
		moves.reverse();
		moves.truncate(4);

		return moves.into();		
	}

    pub fn generate_gender(&self) -> Gender {
        match self.breeding.gender {
            Some(percentage) => if POKEMON_RANDOM.gen_range(0..8) as u8 > percentage {
                Gender::Male
            } else {
                Gender::Female
            }
            None => Gender::None,
        }
    }
	
}