use serde::{Deserialize, Serialize};

use crate::pokemon::Level;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Training {
	
	pub base_exp: u16,
	#[serde(default)]
	pub growth_rate: GrowthRate,
	//pub ev_yield: Option<(String, usize)>,
	//pub catch_rate: Option<u8>,
	//pub base_friendship: Option<u8>,
	
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GrowthRate {

	#[serde(rename = "fast")]
	Fast,
	#[serde(rename = "medium")]
	Medium,
	#[serde(rename = "medium-slow")]
	MediumSlow,
	#[serde(rename = "slow")]
	Slow,
	#[serde(rename = "fast-then-very-slow")]
	FastSlow,
	#[serde(rename = "slow-then-very-fast")]
	SlowFast,
	
}

impl Default for GrowthRate {
    fn default() -> Self {
        Self::MediumSlow
    }
}

impl GrowthRate {

	pub fn max_exp(self, level: Level) -> u32 {
		let level = level as u32;
		match self {
		    GrowthRate::Fast => (0.8 * level.pow(3) as f32) as u32,
		    GrowthRate::Medium => level.pow(3) as u32,
		    GrowthRate::Slow => (1.25 * level.pow(3) as f32) as u32,
			_ => ((1.2 * level.pow(3) as f32) as isize - 15 * level.pow(2) as isize + 100 * level as isize - 140) as u32, // MediumSlow
		}
	}

}