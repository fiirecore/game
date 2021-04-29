use serde::{Deserialize, Serialize};
use super::{ItemId, StackSize, ItemRef};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemStack {
    pub id: ItemId,
    pub count: StackSize,
}

#[derive(Debug)]
pub struct ItemStackInstance {
    pub item: ItemRef,
    pub id: ItemId,
    pub count: StackSize,
    pub count_string: String,
}

impl ItemStack {

    pub fn single(item: ItemId) -> Self {
        Self {
            id: item,
            count: 1,
        }
    }

}