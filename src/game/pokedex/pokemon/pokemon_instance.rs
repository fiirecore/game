use std::fmt::Display;

use crate::engine::game_context::GameContext;
use crate::game::pokedex::move_instance::MoveInstance;
use crate::game::pokedex::pokedex::Pokedex;
use crate::io::data::pokemon::LargeStatSet;
use crate::io::data::pokemon::StatSet;
use crate::io::data::pokemon::moves::pokemon_move::PokemonMove;
use crate::io::data::pokemon::pokemon::Pokemon;

pub struct PokemonInstance {
	
	pub pokemon: Pokemon,
	
	pub level: u8,
//	ability: Ability,

	pub moves: Vec<MoveInstance>,

	pub base: LargeStatSet,
	pub ivs: StatSet,
	pub evs: StatSet,

	pub current_hp: u16,
	
}

impl PokemonInstance {

	pub fn faint(&self) -> bool {
		return self.current_hp == 0;
	}

	pub fn new(pokedex: &Pokedex, pokemon: &Pokemon, ivs: StatSet, level: u8) -> PokemonInstance {

		let evs = StatSet::default();

		let stats = get_stats(pokemon, ivs, evs, level);

		PokemonInstance {
			
			pokemon: pokemon.clone(),
			
			level: level,
			
			moves: PokemonInstance::moves_to_instance(pokedex.moves_from_level(pokemon.data.number, level)),
			
			ivs: ivs,
			
			evs: evs,

			base: stats,

			current_hp: stats.hp,
			
		}

	}
	
	pub fn generate(pokedex: &Pokedex, context: &mut GameContext, pokemon: &Pokemon, min_level: u8, max_level: u8) -> PokemonInstance {
		let level;
		if min_level == max_level {
			level = max_level;
		} else {
			level = context.random.rand_range(min_level as u32..(max_level as u32 + 1)) as u8;
		}

		let ivs = StatSet::iv_random(&mut context.random);

		let evs = StatSet::default();

		let base = get_stats(pokemon, ivs, evs, level);

		PokemonInstance {
			
			pokemon: pokemon.clone(),
			
			level: level,
			
			moves: PokemonInstance::moves_to_instance(pokedex.moves_from_level(pokemon.data.number, level)),
			
			ivs: ivs,
			
			evs: evs,

			base: base,

			current_hp: base.hp,
			
		}
		
	}

	pub fn moves_to_instance(moves: Vec<PokemonMove>) -> Vec<MoveInstance> {
		moves.iter().map(|mv| MoveInstance {
			move_instance: mv.clone(),
			remaining_pp: mv.pp,				
		}).collect()
	}	
	
}

impl Display for PokemonInstance {

	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Lv. {} {}", self.level, &self.pokemon.data.name)
	}
	
}

pub fn get_stats(pokemon: &Pokemon, ivs: StatSet, evs: StatSet, level: u8) -> LargeStatSet {
    LargeStatSet {
		hp: calculate_hp(pokemon.base.hp, ivs.hp, evs.hp, level),
		atk: calculate_stat(pokemon.base.atk, ivs.atk, evs.atk, level),
		def: calculate_stat(pokemon.base.def, ivs.def, evs.def, level),
		sp_atk: calculate_stat(pokemon.base.sp_atk, ivs.sp_atk, evs.sp_atk, level),
		sp_def: calculate_stat(pokemon.base.sp_def, ivs.sp_def, evs.sp_def, level),
		speed: calculate_stat(pokemon.base.speed, ivs.speed, evs.speed, level),
	}
}

pub fn calculate_stat(base_stat: u8, iv_stat: u8, ev_stat: u8, level: u8) -> u16 { //add item check
	let nature = 1.0;
   (((2.0 * base_stat as f64 + iv_stat as f64 + ev_stat as f64) * level as f64 / 100.0 + 5.0).floor() * nature).floor() as u16
}

pub fn calculate_hp(base_hp: u8, iv_hp: u8, ev_hp: u8, level: u8) -> u16 {
   ((2.0 * base_hp as f64 + iv_hp as f64 + ev_hp as f64) * level as f64 / 100.0 + level as f64 + 10.0).floor() as u16
}