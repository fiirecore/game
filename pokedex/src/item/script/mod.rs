use serde::{Deserialize, Serialize};

use crate::{moves::usage::Percent, status::Status};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemScript {
    pub conditions: Option<Vec<ItemCondition>>, // optional because some items cannot be used

    // #[serde(rename = "actions")]
    // original_actions: VecDeque<ItemActionKind>,

    // #[serde(skip)]
    pub actions: Vec<ItemActionKind>, // this should not need to update

    pub consume: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ItemCondition {
    BelowHealthPercent(Percent),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ItemActionKind {
    CurePokemon(Option<Status>),
    HealPokemon(u16),
}
