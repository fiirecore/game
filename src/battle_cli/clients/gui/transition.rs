pub mod opener;
pub mod introduction;

pub mod trainer;

#[derive(Debug)]
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