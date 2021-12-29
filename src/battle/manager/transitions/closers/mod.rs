mod trainer;
mod wild;

pub use trainer::TrainerBattleCloser;
pub use wild::WildBattleCloser;

pub enum Closers {
    Wild,
    Trainer,
}

impl Default for Closers {
    fn default() -> Self {
        Self::Wild
    }
}
