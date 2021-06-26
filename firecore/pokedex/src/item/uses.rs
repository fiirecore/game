use serde::{Deserialize, Serialize};

use super::script::ItemScript;

#[derive(Debug, Deserialize, Serialize)]
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