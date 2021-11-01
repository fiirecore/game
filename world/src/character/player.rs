use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

use super::Character;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerCharacter {
    pub character: Character,
    pub input_frozen: bool,
    pub ignore: bool,
}

impl Deref for PlayerCharacter {
    type Target = Character;

    fn deref(&self) -> &Self::Target {
        &self.character
    }
}

impl DerefMut for PlayerCharacter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.character
    }
}
