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

use deps::str::TinyStr8;

impl BattleTransitions {
    const FLASH: TinyStr8 = unsafe { TinyStr8::new_unchecked(448612363334) }; 
    const TRAINER: TinyStr8 = unsafe { TinyStr8::new_unchecked(32199672233816660) };
}

impl From<TinyStr8> for BattleTransitions {
    fn from(transition: TinyStr8) -> Self {
        match transition {
            Self::FLASH => Self::Flash,
            Self::TRAINER => Self::Trainer,
            _ => Self::default(),
        }
    }
}