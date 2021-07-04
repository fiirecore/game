use serde::{Deserialize, Serialize};

use super::script::ItemScript;

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum ItemUseType {
    Script(ItemScript),
    Pokeball,
    None
}

impl Default for ItemUseType {
    fn default() -> Self {
        Self::None
    }
}