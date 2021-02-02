use crate::pokemon::data::StatSet;
use crate::pokemon::data::PokedexData;
use crate::pokemon::Pokemon;
use crate::pokemon::data::training::Training;
use crate::pokemon::instance::PokemonInstance;
use crate::pokemon::moves::MoveInstance;
use crate::pokemon::pokedex::Pokedex;

pub struct BattlePokemon {
	
	pub data: PokedexData,
	pub training: Training,
	
	pub level: u8,
//	ability: Ability,

	pub moves: Vec<MoveInstance>,

	pub base: BaseStatSet,

	ivs: StatSet,
	evs: StatSet,

	pub current_hp: u16,

	pub exp: usize,
	
}

impl BattlePokemon {

	pub fn faint(&self) -> bool {
		return self.current_hp == 0;
	}

	pub fn new(pokedex: &Pokedex, pokemon: &PokemonInstance) -> Self {

		let pokemon_data = pokedex.pokemon_from_id(pokemon.id);

		let ivs = pokemon.ivs.unwrap_or(StatSet::iv_random());
		let evs = pokemon.evs.unwrap_or_default();

		let stats = get_stats(pokemon_data, ivs, evs, pokemon.level);

		Self {
			
			data: pokemon_data.data.clone(),
			training: pokemon_data.training,
			
			level: pokemon.level,
			
			moves: pokedex.moves_from_level(pokemon.id, pokemon.level),
			
			ivs: ivs,
			
			evs: evs,
			
			current_hp: pokemon.current_hp.unwrap_or(stats.hp),

			base: stats,

			exp: pokemon.exp.unwrap_or_default(),
			
		}

	}
	
	pub fn generate(pokedex: &Pokedex, pokemon: &Pokemon, min_level: u8, max_level: u8) -> Self {
		let level;
		if min_level == max_level {
			level = max_level;
		} else {
			level = macroquad::rand::gen_range(min_level, max_level + 1);
		}

		let ivs = StatSet::iv_random();
		let evs = StatSet::default();

		let base = get_stats(pokemon, ivs, evs, level);

		Self {
			
			data: pokemon.data.clone(),
			training: pokemon.training,
			
			level: level,
			
			moves: pokedex.moves_from_level(pokemon.data.number, level),
			
			ivs: ivs,
			evs: evs,

			base: base,

			current_hp: base.hp,
			exp: 0,
			
		}
		
	}

	pub fn to_instance(&self) -> PokemonInstance {
		PokemonInstance {
		    id: self.data.number,
		    level: self.level,
		    ivs: Some(self.ivs),
		    evs: Some(self.evs),
		    move_set: Some(crate::pokemon::moves::instance::SavedPokemonMoveSet::from_instance(&self.moves)),
		    exp: Some(self.exp),
		    friendship: Some(70),
		    current_hp: Some(self.current_hp),
		}
	}

	// pub fn moves_to_instance(moves: Vec<PokemonMove>) -> Vec<MoveInstance> {
	// 	moves.iter().map(|mv| MoveInstance {
	// 		move_instance: mv.clone(),
	// 		remaining_pp: mv.pp,				
	// 	}).collect()
	// }	
	
}

impl std::fmt::Display for BattlePokemon {

	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Lv. {} {}", self.level, &self.data.name)
	}
	
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, Default)]
pub struct BaseStatSet {

	pub hp: u16,
	pub atk: u16,
	pub def: u16,
	pub sp_atk: u16,
	pub sp_def: u16,
	pub speed: u16,

}

pub fn get_stats(pokemon: &Pokemon, ivs: StatSet, evs: StatSet, level: u8) -> BaseStatSet {
    BaseStatSet {
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