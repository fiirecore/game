mod wild;
mod trainer;

mod manager;

pub use wild::WildBattleCloser;
pub use trainer::TrainerBattleCloser;

pub enum Closers {
    Wild,
    Trainer,
}

impl Default for Closers {
    fn default() -> Self {
        Self::Wild
    }
}