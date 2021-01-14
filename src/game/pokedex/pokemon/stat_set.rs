use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatSet {

	pub hp: u8,
	pub atk: u8,
	pub def: u8,
	pub sp_atk: u8,
	pub sp_def: u8,
	pub speed: u8,

}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LargeStatSet {

	pub hp: u16,
	pub atk: u16,
	pub def: u16,
	pub sp_atk: u16,
	pub sp_def: u16,
	pub speed: u16,

}

impl Default for StatSet {
	fn default() -> Self {
		Self {
			hp: 0,
			atk: 0,
			def: 0,
			sp_atk: 0,
			sp_def: 0,
			speed: 0
		}
	}
}

impl StatSet {

	pub fn iv_random(random: &mut oorandom::Rand32) -> Self {

		Self {
			hp: random.rand_range(0..32) as u8,
			atk: random.rand_range(0..32) as u8,
			def: random.rand_range(0..32) as u8,
			sp_atk: random.rand_range(0..32) as u8,
			sp_def: random.rand_range(0..32) as u8,
			speed: random.rand_range(0..32) as u8,
		}

	}

	pub fn uniform(stat: u8) -> Self {

		Self {
			hp: stat,
			atk: stat,
			def: stat,
			sp_atk: stat,
			sp_def: stat,
			speed: stat,
		}
		
	}

}