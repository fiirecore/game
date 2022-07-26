use std::ops::Deref;

use serde::{Deserialize, Serialize};

use pokedex::{
    item::{Item, ItemId, ItemStack},
    moves::{Move, MoveId},
    pokemon::Pokemon,
    trainer::InitTrainer,
};

pub mod group;
pub use group::*;

use crate::{
    positions::{Coordinate, Location},
    state::map::MapState,
};

pub type ObjectId = u16;
pub type ObjectType = tinystr::TinyAsciiStr<4>;

pub type Objects = hashbrown::HashMap<ObjectId, ObjectEntity>;
/// to - do: item object and item object state
pub type Items = hashbrown::HashMap<ObjectId, ItemEntity>;
pub type Signs = hashbrown::HashMap<ObjectId, SignEntity>;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Entity<E> {
    pub coordinate: Coordinate,
    pub data: E,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct EntityState<E> {
    pub entity: E,
    pub removed: bool,
}

pub type ObjectEntity = Entity<ObjectEntityData>;
pub type ObjectEntityState = EntityState<ObjectEntity>;
pub type ItemEntity = Entity<ItemEntityData>;
pub type ItemEntityState = EntityState<ItemEntity>;
pub type SignEntity = Entity<SignEntityData>;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct ObjectEntityData {
    pub group: ObjectType,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct ItemEntityData {
    pub item: ItemStack<ItemId>,
    pub hidden: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SignEntityData {
    pub message: Vec<Vec<String>>,
}

impl ObjectEntity {
    const TREE: &'static ObjectType =
        unsafe { &ObjectType::from_bytes_unchecked(1701147252u32.to_ne_bytes()) };

    const CUT: &'static MoveId = unsafe {
        &MoveId(tinystr::TinyStr16::from_bytes_unchecked(
            7632227u128.to_ne_bytes(),
        ))
    };

    const ROCK: &'static ObjectType =
        unsafe { &ObjectType::from_bytes_unchecked(1801678706u32.to_ne_bytes()) };
    /// "rock-smash"
    const ROCK_SMASH: &'static MoveId = unsafe {
        &MoveId(tinystr::TinyStr16::from_bytes_unchecked(
            493254510180952753532786u128.to_ne_bytes(),
        ))
    };

    pub fn try_break<
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    >(
        location: &Location,
        coordinate: Coordinate,
        group: &ObjectType,
        trainer: &mut InitTrainer<P, M, I>,
        state: &mut MapState,
        force: bool,
    ) {
        let id = match group {
            Self::ROCK => Self::ROCK_SMASH,
            Self::TREE => Self::CUT,
            _ => return,
        };
        if trainer
            .party
            .iter()
            .any(|p| p.moves.iter().any(|m| m.id() == id))
            || force
        {
            if let Some(objects) = state.entities.get_mut(location) {
                todo!()
            }
        }
    }
}

impl ItemEntity {
    pub fn pickup(&self, location: &Location, coordinate: Coordinate, state: &mut MapState) {
        // if !state.contains_object(location, &coordinate) {
        //     state.insert_object(location, coordinate);
        //     state.events.push(MapEvent::GetItem(self.item));
        // }
        todo!()
    }
}
