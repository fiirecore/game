use serde::{Deserialize, Serialize};
use deps::hash::HashMap;
use super::{ItemId, ItemRef, ItemStack};


#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Bag {
	#[serde(default)]
    pub items: HashMap<ItemId, ItemStack>,
}

impl Bag {

	pub fn add_item(&mut self, stack: ItemStack) -> Option<ItemStack> { // returns extra item
		if let Some(owned) = self.items.get_mut(stack.item.id()) {
			owned.add(stack)
		} else {
			self.items.insert(*stack.item.id(), stack)
		}
	}

	pub fn use_item(&mut self, id: &ItemId) -> Option<ItemRef> {
		self.items.get_mut(id).map(|stack| if stack.decrement() { Some(stack.item) } else { None }).flatten()
	}

}