use serde::{Deserialize, Serialize};
use super::{ItemId, StackSize, ItemRef};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemStack {
    pub item: ItemRef,
    pub count: StackSize,
}

#[derive(Debug)]
pub struct ItemStackInstance<'a> {
    pub stack: &'a mut ItemStack,
    pub count_string: String,
}

impl ItemStack {

    pub fn add(&mut self, stack: ItemStack) -> Option<ItemStack> {
        self.count += stack.count;
        match &self.item {
            crate::Ref::Init(item) => {
                if self.count > item.stack_size {
                    let count = self.count - item.stack_size;
                    self.count = item.stack_size;
                    Some(
                        ItemStack {
                            item: stack.item,
                            count,
                        }
                    )
                } else {
                    None
                }
            }
            crate::Ref::Uninit(_) => None,
        }
    }

    pub fn decrement(&mut self) -> bool {
        if self.count > 0 {
            self.count -= 1;
            true
        } else {
            false
        }
    }

    pub fn single(item: &ItemId) -> Self {
        Self {
            item: <super::Item as crate::Identifiable>::get(item),
            count: 1,
        }
    }
}

impl<'a> ItemStackInstance<'a> {
    pub fn decrement(&mut self) -> bool {
        if self.stack.decrement() {
            self.count_string = self.stack.count.to_string();
            true
        } else {
            false
        }
    }
}