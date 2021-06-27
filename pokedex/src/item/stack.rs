use super::{Item, ItemId, ItemRef, StackSize};
use deps::borrow::{Identifiable, StaticRef};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemStack {
    pub item: ItemRef,
    pub count: StackSize,
}

#[derive(Debug)]
pub struct ItemStackInstance {
    pub stack: *mut ItemStack,              // we do a little trolling
    #[cfg(feature = "stackcount")]
    count: ([u8; 4], Option<StackSize>),    // i think this is fine
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
                    Some(ItemStack {
                        item: stack.item,
                        count,
                    })
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

impl ItemStackInstance {
    pub fn stack(&self) -> &mut ItemStack {
        unsafe { self.stack.as_mut().unwrap() }
    }

    #[cfg(feature = "stackcount")]
    pub fn count(&mut self) -> &str {
        let count = self.stack().count;
        if self.count.1 != Some(count) {
            itoa::write(self.count.0.as_mut(), count).unwrap();
            self.count.1 = Some(count);
        }
        unsafe { std::str::from_utf8_unchecked(&self.count.0) }
    }
}

impl From<&mut ItemStack> for ItemStackInstance {
    fn from(stack: &mut ItemStack) -> Self {
        Self {
            stack: stack as *mut ItemStack,
            #[cfg(feature = "stackcount")]
            count: Default::default(),
        }
    }
}
