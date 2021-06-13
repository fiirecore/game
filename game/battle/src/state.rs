use pokedex::moves::target::PlayerId;

#[derive(Debug)]
pub enum BattleState {
	StartWait,
	Setup,
	Selecting(bool),
	Moving(bool),
	End(bool, PlayerId),
}

impl BattleState {
	pub const SELECTING_START: Self = Self::Selecting(false);
	pub const MOVE_START: Self = Self::Moving(false);
}

impl Default for BattleState {
    fn default() -> Self {
        Self::StartWait
    }
}