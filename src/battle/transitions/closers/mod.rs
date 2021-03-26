pub mod basic;

pub enum Closers {

    Basic,

}

impl Default for Closers {
    fn default() -> Self {
        Self::Basic
    }
}