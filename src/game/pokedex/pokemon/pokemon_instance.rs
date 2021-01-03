use log::warn;

use crate::{engine::game_context::GameContext, game::pokedex::{pokedex::Pokedex, pokemon::pokemon::Pokemon, pokemon_move::move_instance::MoveInstance}};
use crate::game::pokedex::pokemon_move::pokemon_move::PokemonMove;

use super::stat_set::StatSet;

pub struct PokemonInstance {
	
	pub pokemon: Pokemon,
	
	pub level: u8,
//	ability: Ability,

	pub moves: Vec<MoveInstance>,

	pub ivs: StatSet,

	pub evs: StatSet,
	
}

impl PokemonInstance {

	pub fn new(pokedex: &Pokedex, pokemon: &Pokemon, ivs: StatSet, level: u8) -> PokemonInstance {

		PokemonInstance {
			
			pokemon: pokemon.clone(),
			
			level: level,
			
			moves: PokemonInstance::get_moves_from_level(pokedex, pokemon, level).iter().map(|mv| MoveInstance {
			    move_instance: mv.clone(),
			    remaining_pp: mv.pp,				
			}).collect(),
			
			ivs: ivs,
			
			evs: StatSet::default(),
			
		}

	}
	
	pub fn generate(pokedex: &Pokedex, context: &mut GameContext, pokemon: &Pokemon, min_level: u8, max_level: u8) -> PokemonInstance {
		let level;
		if min_level == max_level {
			level = max_level;
		} else {
			level = context.random.rand_range(min_level as u32..(max_level as u32 + 1)) as u8;
		}

		PokemonInstance {
			
			pokemon: pokemon.clone(),
			
			level: level,
			
			moves: PokemonInstance::get_moves_from_level(pokedex, pokemon, level).iter().map(|mv| MoveInstance {
			    move_instance: mv.clone(),
			    remaining_pp: mv.pp,				
			}).collect(),
			
			ivs: StatSet::iv_random(&mut context.random),
			
			evs: StatSet::default(),
			
		}
		
	}

	pub fn get_move(&self, index: usize) -> Option<&MoveInstance> {
		if self.moves.len() > index {
			return Some(&self.moves[index]);
		} else {
			return None;
		}
	}

	pub fn get_moves_from_level(pokedex: &Pokedex, pokemon: &Pokemon, level: u8) -> Vec<PokemonMove> {
		let mut moves = Vec::new();
		for index in 0..level+1 {
			if let Some(pkmn_move_str) = pokemon.learnable_moves.get(&index) {
				for string in pkmn_move_str {
					match pokedex.move_list.get(string) {
						Some(pokemon_move) => {
							moves.push(pokemon_move.clone());
						}
						None => {
							warn!("Could not add pokemon move {} to {}", string, pokemon.name)
						}
					}
				}								
			}
		}
		while moves.len() > 4 {
			moves.remove(0);
		}
		return moves;
	}
	
	
}