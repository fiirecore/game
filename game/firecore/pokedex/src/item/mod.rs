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

pub const ITEM_UNKNOWN: ItemId = unsafe { ItemId::new_unchecked(31093567915781749) };

impl Identifiable for Item {

    type Id = ItemId;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn try_get(id: &Self::Id) -> Option<&'static Self> {
        unsafe { dex::ITEMDEX.as_ref().expect("Itemdex was not initialized!").get(id) }
    }

    fn unknown() -> Option<&'static Self> {
        Self::try_get(&ITEM_UNKNOWN)
    }

}

pub const fn default_stack_size() -> StackSize {
    999
}

