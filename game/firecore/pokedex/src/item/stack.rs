use serde::{Deserialize, Serialize};
use deps::borrow::{
    Identifiable,
    StaticRef,
};
use super::{ItemId, Item, ItemRef, StackSize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemStack {
    pub item: ItemRef,
    pub count: StackSize,
}

#[derive(Debug)]
pub struct ItemStackInstance<'a> {
    pub stack: &'a mut ItemStack,
    pub count_string: String, // To - do: research into not creating string for count
}

impl ItemStack {

    pub fn new(item: &ItemId, count: StackSize) -> Self {
        Self {
            item: Item::get(item),
            count,
        }
    }

    pub fn add(&mut self, stack: ItemStack) -> Option<ItemStack> {
        self.count += stack.count;
        match &self.item {
            StaticRef::Init(item) => {
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
            StaticRef::Uninit(_) => None,
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