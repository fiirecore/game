use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Breeding {
	
	// pub groups: Vec<EggGroup>,
	pub gender: Option<u8>, // None = no gender, 0 = 100% female, 7 = 100% male (0-8 scale)
	// pub cycles: Option<u8>,
	
}