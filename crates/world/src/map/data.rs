use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use pokedex::{item::ItemId, moves::MoveId, trainer::InitTrainer};

use crate::{
    character::{
        npc::{
            group::{NpcGroup, TrainerGroup, TrainerGroupId},
            trainer::TrainerDisable,
        },
        Capability, CharacterGroupId, CharacterState,
    },
    positions::{Coordinate, Direction, Location, Spot},
    state::map::MapState,
};

use self::tile::PaletteDataMap;

use super::{chunk::Connection, warp::WarpDestination, wild::WildChances, MovementId, WorldMap};

pub mod tile;

pub type WorldMaps = HashMap<Location, WorldMap>;
pub type FieldMoveData = HashMap<MoveId, FieldType>;
pub type FieldItemData = HashMap<ItemId, FieldType>;
// pub type ObjectData = HashMap<ObjectType, Removable>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorldMapData {
    pub maps: WorldMaps,
    // pub objects: ObjectData,
    pub palettes: PaletteDataMap,
    pub npc: WorldNpcData,
    pub wild: WildChances,
    pub moves: FieldMoveData,
    pub items: FieldItemData,
    pub spawn: Spot,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorldNpcData {
    pub groups: HashMap<CharacterGroupId, NpcGroup>,
    pub trainers: HashMap<TrainerGroupId, TrainerGroup>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum FieldType {
    Capability(Capability),
}

impl WorldMapData {
    pub fn update_capabilities(&self, character: &mut CharacterState, trainer: &mut InitTrainer) {
        fn set(can: bool, character: &mut CharacterState, t: &FieldType) {
            match can {
                true => match t {
                    FieldType::Capability(capability) => {
                        character.capabilities.insert(*capability);
                    }
                },
                false => match t {
                    FieldType::Capability(capability) => {
                        character.capabilities.remove(capability);
                    }
                },
            }
        }

        for (id, t) in self.moves.iter() {
            set(
                trainer
                    .party
                    .iter()
                    .any(|p| p.moves.iter().any(|m| &m.0.id == id)),
                character,
                t,
            );
        }

        for (id, t) in self.items.iter() {
            set(
                trainer.bag.iter().any(|stack| &stack.item.id == id),
                character,
                t,
            );
        }
    }

    pub fn connection_movement(
        &self,
        direction: Direction,
        offset: i32,
        connections: &[Connection],
        state: &MapState,
    ) -> Option<(Location, Coordinate, MovementId)> {
        connections.iter().find_map(|connection| {
            self.maps
                .get(&connection.0)
                .map(|map| {
                    let o = offset - connection.1;
                    let position = Connection::offset(direction, map, o);
                    let coords = position.in_direction(direction);
                    map.local_movement(coords, state)
                        .map(|code| (map.id, position, code))
                })
                .flatten()
        })
    }

    pub fn post_battle(&self, state: &mut MapState, trainer: &mut InitTrainer, winner: bool) {
        state.player.character.locked.decrement();
        let entries = std::mem::take(&mut state.player.battle.battling);
        if winner {
            for entry in entries {
                if let Some(trainer) = entry.trainer {
                    state.player.character.end_interact();
                    if let Some(character) = state
                        .entities
                        .get_mut(&state.location)
                        .and_then(|state| state.npcs.get_mut(&trainer.id))
                    {
                        character.end_interact();
                    }
                    if let Some(npc) = self
                        .maps
                        .get(&trainer.location)
                        .and_then(|map| map.npcs.get(&trainer.id))
                        .and_then(|npc| npc.trainer.as_ref())
                    {
                        match &npc.disable {
                            TrainerDisable::DisableSelf => {
                                state.player.battle.insert(&trainer.location, trainer.id);
                            }
                            TrainerDisable::Many(others) => {
                                state.player.battle.insert(&trainer.location, trainer.id);
                                state
                                    .player
                                    .battle
                                    .battled
                                    .get_mut(&trainer.location)
                                    .unwrap()
                                    .extend(others);
                            }
                        }
                    }
                }
            }
        } else {
            self.whiteout(state, trainer);
        }
        self.update_capabilities(&mut state.player.character, trainer);
    }

    pub fn whiteout(&self, state: &mut MapState, trainer: &mut InitTrainer) {
        let Spot { location, position } = state.places.heal.unwrap_or(self.spawn);
        state.location = location;
        state.player.character.position = position;
        trainer.party.iter_mut().for_each(|o| o.heal(None, None));
    }

    pub fn warp(&self, state: &mut MapState, destination: WarpDestination) -> bool {
        match self.maps.contains_key(&destination.location) {
            true => {
                MapState::warp(
                    &mut state.location,
                    &mut state.player.character,
                    destination,
                );
                true
            }
            false => false,
        }
    }
}
