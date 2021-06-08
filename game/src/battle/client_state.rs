#[derive(Debug, PartialEq)]
pub enum BattleManagerState {
	Begin,
	Transition,
	Opener,
	Introduction,
	Battle,
	Closer,
}

impl Default for BattleManagerState {
    fn default() -> Self {
        Self::Begin
    }
}

#[derive(PartialEq)]
pub enum TransitionState {
	Begin, // runs on spawn methods
	Run,
	End, // spawns next state and goes back to beginning
}

impl Default for TransitionState {
    fn default() -> Self {
        Self::Begin
    }
}