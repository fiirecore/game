use super::{ItemId, ItemRef, ItemStack};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Bag {
    #[serde(default)]
    pub items: Vec<ItemStack>,
}

impl Bag {
    pub fn add_item(&mut self, stack: ItemStack) -> Option<ItemStack> {
        // returns extra item
        match self
            .items
            .iter()
            .position(|stack2| stack2.item.id() == stack.item.id())
        {
            Some(pos) => self.items[pos].add(stack),
            None => {
                self.items.push(stack);
                None
            }
        }
    }

    pub fn position(&self, id: &ItemId) -> Option<usize> {
        self.items.iter().position(|stack| stack.item.id() == id)
    }

    pub fn use_item(&mut self, id: &ItemId) -> Option<ItemRef> {
        self.position(id)
            .map(|id| {
                let stack = &mut self.items[id];
                stack.decrement().then(|| stack.item)
            })
            .flatten()
    }
}
