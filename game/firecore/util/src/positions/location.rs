use serde::{Deserialize, Serialize};
use firecore_dependencies::tinystr::TinyStr16;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Location {

	pub map: Option<TinyStr16>,
	pub index: TinyStr16,

}