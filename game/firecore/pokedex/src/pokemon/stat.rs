use serde::{Deserialize, Serialize};

use super::{POKEMON_RANDOM, Pokemon, Level};

pub type Stat = u8;
pub type BaseStat = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StatSet {
	pub hp: Stat,
	pub atk: Stat,
	pub def: Stat,
	pub sp_atk: Stat,
	pub sp_def: Stat,
	pub speed: Stat,
}

impl StatSet {

	pub const MAX_EV: Stat = 32;
	pub const MAX_IV: Stat = 252;
	pub const MAX_IVS_TOTAL: u16 = 512;

	pub const fn uniform(stat: Stat) -> Self {
		Self {
			hp: stat,
			atk: stat,
			def: stat,
			sp_atk: stat,
			sp_def: stat,
			speed: stat,
		}
	}

	pub fn random() -> Self {
		Self {
			hp: POKEMON_RANDOM.gen_range(0, Self::MAX_EV) as u8,
			atk: POKEMON_RANDOM.gen_range(0, Self::MAX_EV) as u8,
			def: POKEMON_RANDOM.gen_range(0, Self::MAX_EV) as u8,
			sp_atk: POKEMON_RANDOM.gen_range(0, Self::MAX_EV) as u8,
			sp_def: POKEMON_RANDOM.gen_range(0, Self::MAX_EV) as u8,
			speed: POKEMON_RANDOM.gen_range(0, Self::MAX_EV) as u8,
		}
	}

}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct BaseStatSet {

	pub hp: BaseStat,
	pub atk: BaseStat,
	pub def: BaseStat,
	pub sp_atk: BaseStat,
	pub sp_def: BaseStat,
	pub speed: BaseStat,

}

impl BaseStatSet {

	pub fn get(pokemon: &Pokemon, ivs: StatSet, evs: StatSet, level: Level) -> Self {
		Self {
			hp: calculate_hp(pokemon.base.hp, ivs.hp, evs.hp, level),
			atk: calculate_stat(pokemon.base.atk, ivs.atk, evs.atk, level),
			def: calculate_stat(pokemon.base.def, ivs.def, evs.def, level),
			sp_atk: calculate_stat(pokemon.base.sp_atk, ivs.sp_atk, evs.sp_atk, level),
			sp_def: calculate_stat(pokemon.base.sp_def, ivs.sp_def, evs.sp_def, level),
			speed: calculate_stat(pokemon.base.speed, ivs.speed, evs.speed, level),
		}
	}

}

pub fn calculate_stat(base: Stat, iv: Stat, ev: Stat, level: Level) -> BaseStat { //add item check
    let nature = 1.0;
    (((2.0 * base as f32 + iv as f32 + ev as f32) * level as f32 / 100.0 + 5.0).floor() * nature).floor() as BaseStat
}

pub fn calculate_hp(base: Stat, iv: Stat, ev: Stat, level: Level) -> BaseStat {
    ((2.0 * base as f64 + iv as f64 + ev as f64) * level as f64 / 100.0 + level as f64 + 10.0).floor() as BaseStat
}