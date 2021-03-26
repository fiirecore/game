use super::introductions::Introductions;

pub mod trainer;
pub mod wild;

pub enum Openers {

    Wild,
    Trainer,

}

impl Default for Openers {
    fn default() -> Self {
        Self::Wild
    }
}

impl Openers {

    pub fn intro(&self) -> Introductions {
        match self {
            Openers::Wild => Introductions::Basic,
            Openers::Trainer => Introductions::Trainer,
        }
    }

}