use crate::pokemon::data::StatSet;
use crate::pokemon::data::PokedexData;
use crate::pokemon::Pokemon;
use crate::pokemon::data::training::Training;
use crate::pokemon::instance::PokemonInstance;
use crate::pokemon::moves::instance::MoveInstances;

pub struct BattlePokemon {
	
	pub data: PokedexData,
	pub training: Training,
	
	pub nickname: Option<String>,
	pub level: u8,
//	ability: Ability,

	pub moves: MoveInstances,

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

	pub fn new(pokemon: &PokemonInstance) -> Self {

		let pokedex = &crate::pokemon::pokedex::POKEDEX;

		let pokemon_data = pokedex.get(&pokemon.id).expect("Could not get Pokemon from id!");
		let pokemon_data = pokemon_data.value();

		let stats = get_stats(pokemon_data, pokemon.ivs, pokemon.evs, pokemon.level);

		Self {
			
			data: pokemon_data.data.clone(),
			training: pokemon_data.training,
			
			nickname: pokemon.nickname.clone(),
			level: pokemon.level,
			
			moves: pokemon_data.moves_from_level(pokemon.level),
			
			ivs: pokemon.ivs,
			
			evs: pokemon.evs,
			
			current_hp: pokemon.current_hp.unwrap_or(stats.hp),

			base: stats,

			exp: pokemon.exp,
			
		}

	}
	
	pub fn generate(pokemon: &Pokemon, min_level: u8, max_level: u8) -> Self {
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
			
			nickname: None,
			level: level,
			
			moves: pokemon.moves_from_level(level),
			
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
			nickname: self.nickname.clone(),
		    level: self.level,
		    ivs: self.ivs,
		    evs: self.evs,
		    moves: Some(crate::pokemon::moves::serializable::from_instances(&self.moves)),
		    exp: self.exp,
		    friendship: 70,
		    current_hp: Some(self.current_hp),
		}
	}	
	
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