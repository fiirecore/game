pub mod basic;
pub mod trainer;

pub mod util {
    pub mod player_intro;
}

pub enum Introductions {

    Basic,
    Trainer,

}

impl Default for Introductions {
    fn default() -> Self {
        Self::Basic
    }
}