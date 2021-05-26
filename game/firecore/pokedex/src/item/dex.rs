use deps::hash::HashMap;
use super::{Item, ItemId};

pub type Itemdex = HashMap<ItemId, Item>;

pub(crate) static mut ITEMDEX: Option<Itemdex> = None;

pub fn set(dex: Itemdex) {
    unsafe { ITEMDEX = Some(dex) }
}