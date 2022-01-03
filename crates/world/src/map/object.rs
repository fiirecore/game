use serde::{Deserialize, Serialize};

use pokedex::item::{ItemId, ItemStack};

pub mod group;
pub use group::*;

use crate::positions::Coordinate;

pub type ObjectId = tinystr::TinyStr8;

pub type Objects = hashbrown::HashMap<Coordinate, MapObject>;
pub type Items = hashbrown::HashMap<Coordinate, ItemObject>;
pub type Signs = hashbrown::HashMap<Coordinate, SignObject>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MapObject {
    pub group: ObjectId,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemObject {
    pub item: ItemStack<ItemId>,
    pub hidden: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SignObject {
    pub message: Vec<Vec<String>>,
}