use serde::{Deserialize, Serialize};

use deps::{
    str::TinyStr16,
    borrow::{
        Identifiable,
        StaticRef,
    },
};

pub mod dex;

mod stack;
pub use stack::*;

mod uses;
pub use uses::*;

pub mod script;

pub mod bag;

pub type ItemId = TinyStr16;
pub type StackSize = u16;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Item {

    pub id: ItemId,

    pub name: String,
    pub description: Vec<String>,

    #[serde(default = "default_stack_size")]
    pub stack_size: StackSize,

    #[serde(default, rename = "use")]
    pub usage: ItemUseType,

}

pub type ItemRef = StaticRef<Item>;

impl Identifiable for Item {

    type Id = ItemId;

    const UNKNOWN: ItemId = unsafe { ItemId::new_unchecked(31093567915781749) };

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn try_get(id: &Self::Id) -> Option<&'static Self> {
        unsafe { dex::ITEMDEX.as_ref().expect("Itemdex was not initialized!").get(id) }
    }

}

pub const fn default_stack_size() -> StackSize {
    999
}

