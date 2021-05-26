use serde::{Deserialize, Serialize};

use crate::pokemon::{
    Pokemon, Level,
};

use super::{Stat, Stats, StatSet, StatType};

pub type BaseStat = u16;
pub type Stage = i8;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct BaseStats {
	pub stats: StatSet<BaseStat>,
	pub stages: StatSet<i8>,
}

impl BaseStats {

	pub fn new(pokemon: &Pokemon, ivs: &Stats, evs: &Stats, level: Level) -> Self {
		Self {
			stats: StatSet::new(pokemon, ivs, evs, level),
			stages: StatSet::default(),
		}
	}

    pub fn get(&self, stat: StatType) -> BaseStat {
        StatSet::mult(self.stages.get(stat), self.stats.get(stat))
    }

    pub fn hp(&self) -> BaseStat {
        self.stats.hp
    }

	pub fn can_change_stage(&self, stat: StatType, stage: Stage) -> bool {
		self.stages.get(stat).abs() + stage < 6
	}

	pub fn change_stage(&mut self, stat: StatType, stage: Stage) {
		*self.stages.get_mut(stat) += stage;
	}

}

impl StatSet<BaseStat> {

	pub fn new(pokemon: &Pokemon, ivs: &Stats, evs: &Stats, level: Level) -> Self {
		Self {
			hp: Self::hp(pokemon.base.hp, ivs.hp, evs.hp, level),
			atk: Self::stat(pokemon.base.atk, ivs.atk, evs.atk, level),
			def: Self::stat(pokemon.base.def, ivs.def, evs.def, level),
			sp_atk: Self::stat(pokemon.base.sp_atk, ivs.sp_atk, evs.sp_atk, level),
			sp_def: Self::stat(pokemon.base.sp_def, ivs.sp_def, evs.sp_def, level),
			speed: Self::stat(pokemon.base.speed, ivs.speed, evs.speed, level),
		}
	}

	pub fn stat(base: Stat, iv: Stat, ev: Stat, level: Level) -> BaseStat { //add item check
		let nature = 1.0;
		(((2.0 * base as f32 + iv as f32 + ev as f32) * level as f32 / 100.0 + 5.0).floor() * nature).floor() as BaseStat
	}

	pub fn hp(base: Stat, iv: Stat, ev: Stat, level: Level) -> BaseStat {
		((2.0 * base as f64 + iv as f64 + ev as f64) * level as f64 / 100.0 + level as f64 + 10.0).floor() as BaseStat
	}

}

impl StatSet<Stage> {

    pub fn mult(stage: Stage, base: BaseStat) -> BaseStat {
        base * (2.max(2 + stage) as BaseStat) / (2.max(2 - stage) as BaseStat)
    }

}