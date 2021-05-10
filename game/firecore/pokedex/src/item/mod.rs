use serde::{Deserialize, Serialize};
use deps::{
    hash::HashMap,
    tinystr::TinyStr16,
};
use script::ItemScript;

pub type Itemdex = HashMap<ItemId, Item>;

pub static mut ITEMDEX: Option<Itemdex> = None;

pub fn itemdex() -> &'static Itemdex {
	unsafe { ITEMDEX.as_ref().expect("Itemdex was not initialized!") }
}

mod stack;
pub use stack::*;

pub mod script;

pub type ItemId = TinyStr16;
pub type StackSize = u16;

pub type ItemRef = &'static Item;

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {

    pub id: ItemId,

    pub name: String,
    pub description: Vec<String>,

    #[serde(default = "default_stack_size")]
    pub stack_size: StackSize,

    pub script: ItemScript,

}

pub const fn default_stack_size() -> StackSize {
    999
}