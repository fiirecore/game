pub mod wild;
pub mod trainer;

pub enum Closers {

    Wild,
    Trainer,

}

impl Default for Closers {
    fn default() -> Self {
        Self::Wild
    }
}