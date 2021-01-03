#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum MoveCategory {
	
	Physical,
	Special,
	Status,	
	
}

impl MoveCategory {

	pub fn from_string(string: &str) -> Option<MoveCategory> {
		match string {
			"Physical" => Some(MoveCategory::Physical),
			"Special" => Some(MoveCategory::Special),
			"Status" => Some(MoveCategory::Status),
			&_ => None,
		}
	}

}