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

pub type Objects = hashbrown::HashMap<ObjectId, MapObject>;

/// Map Objects should have scripts attached to them
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct MapObject {
    pub coordinate: Coordinate,
    /// Some = shown
    /// None = hidden
    pub group: Option<ObjectType>,
}

pub enum Removable {}

// impl ObjectEntity {
//     const TREE: &'static ObjectType =
//         unsafe { &ObjectType::from_bytes_unchecked(1701147252u32.to_ne_bytes()) };

//     const CUT: &'static MoveId = unsafe {
//         &MoveId(tinystr::TinyStr16::from_bytes_unchecked(
//             7632227u128.to_ne_bytes(),
//         ))
//     };

//     const ROCK: &'static ObjectType =
//         unsafe { &ObjectType::from_bytes_unchecked(1801678706u32.to_ne_bytes()) };
//     /// "rock-smash"
//     const ROCK_SMASH: &'static MoveId = unsafe {
//         &MoveId(tinystr::TinyStr16::from_bytes_unchecked(
//             493254510180952753532786u128.to_ne_bytes(),
//         ))
//     };

//     pub fn try_break<
//     >(
//         location: &Location,
//         coordinate: Coordinate,
//         group: &ObjectType,
//         trainer: &mut InitTrainer,
//         state: &mut MapState,
//         force: bool,
//     ) {
//         let id = match group {
//             Self::ROCK => Self::ROCK_SMASH,
//             Self::TREE => Self::CUT,
//             _ => return,
//         };
//         if trainer
//             .party
//             .iter()
//             .any(|p| p.moves.iter().any(|m| m.id() == id))
//             || force
//         {
//             if let Some(objects) = state.entities.get_mut(location) {
//                 todo!()
//             }
//         }
//     }
// }

// impl ItemEntity {
//     pub fn pickup(&self, location: &Location, coordinate: Coordinate, state: &mut MapState) {
//         // if !state.contains_object(location, &coordinate) {
//         //     state.insert_object(location, coordinate);
//         //     state.events.push(MapEvent::GetItem(self.item));
//         // }
//         todo!()
//     }
// }
