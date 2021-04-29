use serde::{Deserialize, Serialize};
use firecore_dependencies::tinystr::TinyStr16;

use crate::Position;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Location {

	pub map: Option<TinyStr16>,
	pub index: TinyStr16,

	pub position: Position,

}