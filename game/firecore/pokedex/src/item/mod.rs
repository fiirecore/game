use serde::{Deserialize, Serialize};
use deps::tinystr::TinyStr16;
use script::ItemScript;

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