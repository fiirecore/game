use serde::{Deserialize, Serialize};

use deps::{
    str::TinyStr16,
    hash::HashMap,
    borrow::{Identifiable, StaticRef},
    UNKNOWN16,
};

use crate::Dex;

mod stack;
mod uses;
pub mod script;
pub mod bag;

pub use stack::*;
pub use uses::*;

pub type ItemId = TinyStr16;
pub type StackSize = u16;

pub struct Itemdex;

static mut ITEMDEX: Option<HashMap<ItemId, Item>> = None;

impl Dex<'static> for Itemdex {
    type DexType = Item;

    fn dex() -> &'static mut Option<HashMap<<<Self as Dex<'static>>::DexType as Identifiable<'static>>::Id, Self::DexType>> {
        unsafe { &mut ITEMDEX }
    }
}

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

impl<'a> Identifiable<'a> for Item {

    type Id = ItemId;

    const UNKNOWN: ItemId = UNKNOWN16;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn try_get(id: &Self::Id) -> Option<&'a Self> {
        Itemdex::try_get(id)
    }

}

pub const fn default_stack_size() -> StackSize {
    999
}

