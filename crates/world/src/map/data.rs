use std::ops::Deref;

use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use crate::{positions::{Location, Spot, Direction, Coordinate}, character::{CharacterGroupId, npc::{group::{TrainerGroupId, TrainerGroup, NpcGroup}, trainer::TrainerDisable}}, state::map::MapState};

use self::tile::PaletteDataMap;

use super::{WorldMap, wild::WildChances, chunk::Connection, MovementId, warp::WarpDestination, object::ObjectType};

pub mod tile;

pub type Maps = HashMap<Location, WorldMap>;
// pub type ObjectData = HashMap<ObjectType, Removable>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorldMapData {
    pub maps: Maps,
    // pub objects: ObjectData,
    pub palettes: PaletteDataMap,
    pub npc: WorldNpcData,
    pub wild: WildChances,
    pub spawn: Spot,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorldNpcData {
    pub groups: HashMap<CharacterGroupId, NpcGroup>,
    pub trainers: HashMap<TrainerGroupId, TrainerGroup>,
}

impl WorldMapData {
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

    pub fn post_battle<
        P: Deref<Target = firecore_pokedex::pokemon::Pokemon> + Clone,
        M: Deref<Target = firecore_pokedex::moves::Move> + Clone,
        I: Deref<Target = firecore_pokedex::item::Item> + Clone,
    >(
        &self,
        state: &mut MapState,
        trainer: &mut firecore_pokedex::trainer::InitTrainer<P, M, I>,
        winner: bool,
    ) {
        state.player.character.locked.decrement();
        let entry = state.player.battle.battling.take();
        if winner {
            if let Some(entry) = entry {
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
            let Spot { location, position } = state.places.heal.unwrap_or(self.spawn);
            state.location = location;
            state.player.character.position = position;
            trainer.party.iter_mut().for_each(|o| o.heal(None, None));
        }
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
