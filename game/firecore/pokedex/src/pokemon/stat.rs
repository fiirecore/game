use serde::{Deserialize, Serialize};

use crate::RANDOM;

pub type Stat = u8;

pub type Stats = StatSet<Stat>;

mod base;
pub use base::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatType {
	Attack,
	Defense,
	SpAttack,
	SpDefense,
	Speed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct StatSet<S> {
	pub hp: S,
	pub atk: S,
	pub def: S,
	pub sp_atk: S,
	pub sp_def: S,
	pub speed: S,
}

impl<S: Sized + Copy> StatSet<S> {

	pub fn uniform(stat: S) -> Self {
		Self {
			hp: stat,
			atk: stat,
			def: stat,
			sp_atk: stat,
			sp_def: stat,
			speed: stat,
		}
	}

	pub fn get(&self, stat: StatType) -> S {
		match stat {
			StatType::Attack => self.atk,
			StatType::Defense => self.def,
			StatType::SpAttack => self.sp_atk,
			StatType::SpDefense => self.sp_def,
			StatType::Speed => self.speed,
		}
	}

	pub fn get_mut(&mut self, stat: StatType) -> &mut S {
		match stat {
			StatType::Attack => &mut self.atk,
			StatType::Defense => &mut self.def,
			StatType::SpAttack => &mut self.sp_atk,
			StatType::SpDefense => &mut self.sp_def,
			StatType::Speed => &mut self.speed,
		}
	}

}

impl Stats {

	pub const MAX_EV: Stat = 32;
	pub const MAX_IV: Stat = 252;
	pub const MAX_IVS_TOTAL: u16 = 512;

	pub fn random() -> Self {
		Self {
			hp: RANDOM.gen_range(0, Self::MAX_EV),
			atk: RANDOM.gen_range(0, Self::MAX_EV),
			def: RANDOM.gen_range(0, Self::MAX_EV),
			sp_atk: RANDOM.gen_range(0, Self::MAX_EV),
			sp_def: RANDOM.gen_range(0, Self::MAX_EV),
			speed: RANDOM.gen_range(0, Self::MAX_EV),
		}
	}

}