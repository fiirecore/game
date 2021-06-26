#[derive(Debug)]
pub enum BattleState<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord> {
	StartWait,
	Setup,
	Selecting(bool),
	Moving(bool),
	End(bool, ID),
}

impl<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord> BattleState<ID> {
	pub const SELECTING_START: Self = Self::Selecting(false);
	pub const MOVE_START: Self = Self::Moving(false);
}

impl<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord> Default for BattleState<ID> {
    fn default() -> Self {
        Self::StartWait
    }
}