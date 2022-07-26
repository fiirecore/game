mod flash;
mod trainer;
// pub mod vertical;

pub use flash::FlashBattleTransition;
pub use trainer::TrainerBattleTransition;

#[derive(Debug, Clone, Copy)]
pub enum BattleTransitions {
    Flash,
    Trainer,
}

impl Default for BattleTransitions {
    fn default() -> Self {
        Self::Flash
    }
}

use worldcli::worldlib::map::TransitionId;

impl BattleTransitions {
    const FLASH: TransitionId =
        unsafe { TransitionId::from_bytes_unchecked(448612363334u64.to_ne_bytes()) };
    const TRAINER: TransitionId =
        unsafe { TransitionId::from_bytes_unchecked(32199672233816660u64.to_ne_bytes()) };
}

impl From<TransitionId> for BattleTransitions {
    fn from(transition: TransitionId) -> Self {
        match transition {
            Self::FLASH => Self::Flash,
            Self::TRAINER => Self::Trainer,
            _ => Self::default(),
        }
    }
}
