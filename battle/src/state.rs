#[derive(Debug)]
pub enum BattleState<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord> {
	StartWait,
	Setup,
	StartSelecting,
	WaitSelecting,
	StartMoving,
	WaitMoving,
	End(ID),
}

impl<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord> Default for BattleState<ID> {
    fn default() -> Self {
        Self::StartWait
    }
}