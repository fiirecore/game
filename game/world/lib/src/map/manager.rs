use serde::{Deserialize, Serialize};
use deps::hash::{HashMap, HashSet};
use util::{Direction, Coordinate, Location};
use crate::{
    character::{
        MoveType,
        player::PlayerCharacter,
        npc::NPCId,
    },
    map::{
        TileId,
        MovementId,
        World,
        WorldMap,
        WarpDestination,
    }
};

// pub mod constants;

pub enum TryMoveResult {
    MapUpdate,
    TrySwim,
}

pub type Maps = HashMap<Location, WorldMap>;

#[derive(Deserialize, Serialize)]
pub struct WorldMapManager {

    pub maps: Maps,
    pub data: WorldMapManagerData,

}

#[derive(Default, Deserialize, Serialize)]
pub struct WorldMapManagerData {

    // pub constants: constants::WorldMapManagerConstants,

    #[serde(skip)]
    pub current: Option<Location>,

    #[serde(skip)]
    pub player: PlayerCharacter,

    #[serde(skip)]
    pub warp: Option<WarpDestination>,

    #[serde(skip)]
    pub battling: Option<TrainerEntry>,

    #[serde(skip)]
    pub door: Option<Door>,
}

pub struct TrainerEntry {
    pub map: Location,
    pub id: NPCId,
    pub disable_others: HashSet<NPCId>,
}

pub type TrainerEntryRef<'a> = &'a mut Option<TrainerEntry>;

pub struct Door {
    pub position: usize,
    pub tile: TileId,
    pub accumulator: f32,
    pub open: bool,
}

impl Door {
    pub const DOOR_MAX: f32 = 3.99;
}

impl World for WorldMapManager {
    fn in_bounds(&self, coords: Coordinate) -> bool {
        self.get().map(|map| map.in_bounds(coords)).unwrap_or_default()
    }

    fn tile(&self, coords: Coordinate) -> Option<TileId> {
        self.get().map(|map| map.tile(coords)).flatten()
    }

    fn movement(&self, coords: Coordinate) -> Option<MovementId> {
        self.get().map(|map| map.movement(coords)).flatten()
    }

    fn warp_at(&self, coords: Coordinate) -> Option<&WarpDestination> {
        self.get().map(|map| map.warp_at(coords)).flatten()
    }
}

impl WorldMapManager {

    pub fn get(&self) -> Option<&WorldMap> {
        self.data.current.as_ref().map(|id| self.maps.get(id)).flatten()
    }

    pub fn try_move(&mut self, direction: Direction, delta: f32) -> Option<TryMoveResult> { // return chunk update

        // let mut update = false;

        self.data.player.character.on_try_move(direction);

        let offset = direction.tile_offset();
        let coords = self.data.player.character.position.coords + offset;

        let move_code = self.movement(coords).unwrap_or_else(|| self.walk_connections(coords).unwrap_or(1));

        let warp = match self.data.warp.is_none() {
            true => {
                if let Some(destination) = self.warp_at(coords) {
                    if !destination.transition.warp_on_tile {
                        self.data.warp = Some(*destination);
                        return Some(TryMoveResult::MapUpdate);
                    } else {

                        // open door on warp

                        let map = self.get().unwrap();
                        self.data.door = Some(
                            Door {
                                position: coords.x as usize + coords.y as usize * map.width,
                                tile: map.tile(coords).unwrap(),
                                accumulator: 0.0,
                                open: true,
                            }
                        );
                        self.data.player.character.update_sprite();

                        // door open end

                        true
                    }
                } else {
                    false
                }
            },
            false => false,
        };

        let walk = self.tile(coords).map(|tile_id| match direction {
            Direction::Up => false,
            Direction::Down => match tile_id  {
                135 | 176 | 177 | 143 | 151 | 184 | 185 | 192 | 193 | 217 | 1234 => true,
                _ => false,
            },
            Direction::Left => tile_id == 133,
            Direction::Right => tile_id == 134,
        }).unwrap_or_default();
        
        let allow = warp || walk;

        if self.data.player.character.move_type == MoveType::Swimming && can_walk(move_code) {
            self.data.player.character.move_type = MoveType::Walking
        }

        if can_move(self.data.player.character.move_type, move_code) || allow || self.data.player.character.noclip {
            let mult = self.data.player.character.speed() * 60.0 * delta;
            self.data.player.character.position.offset = direction.pixel_offset().scale(mult);
            self.data.player.character.moving = true;
        } else if can_swim(move_code) && self.data.player.character.move_type != MoveType::Swimming {
            return Some(TryMoveResult::TrySwim);
        }
        None
    }

    pub fn walk_connections(&mut self, coords: Coordinate) -> Option<MovementId> {
        if let Some(current) = self.get() {
            if let Some(chunk) = &current.chunk {
                let current_coords = chunk.coords;
                let absolute = current_coords + coords;
                for connection in chunk.connections.iter() {
                    if let Some(current) = self.maps.get(connection) {
                        if let Some(chunk) = &current.chunk {
                            if let Some(movement) = current.movement(absolute - chunk.coords) {
                                let c = current_coords - chunk.coords;
                                self.data.current = Some(*connection);
                                self.data.player.character.position.coords += c;
                                return Some(movement);
                            }
                        }
                    }
                }
                None
                // if let Some((connection_id, connection, move_id)) = chunk.connections.iter().map(|id| self.maps.get(&Location::new(None, *id)).map(|map| map.movement(absolute).map(|move_id| (id, map, move_id)))).flatten().flatten().find(|(_, _, id)| crate::map::manager::can_move(self.player.character.move_type, *id)) {
                //     let c = current_coords - connection.chunk.as_ref().unwrap().coords;
                //     self.current = Some(Location::new(None, *connection_id));
                //     self.player.character.position.coords += c;
                //     Some(move_id)
                // } else {
                //     None
                // }
            } else {
                None
            }
        } else {
            None
        }
        // self.get().map(|map| map.chunk.as_ref().map(|chunk| {
        //     for connection_id in &chunk.connections {
        //         if let Some(connection) = self.maps.get(connection_id) {
        //             let connection_coords = absolute - current_coords;
        //             move_code = connection.movement(connection_coords);
        //             if let Some(move_code) = move_code {
        //                 if crate::map::manager::can_move(self.player.character.move_type, move_code) {
        //                 }
        //                 break;
        //             }
        //         }
        //     }
        //     move_code
        // })).flatten().flatten()
        
    }

    pub fn warp(&mut self, destination: WarpDestination) {
        match self.maps.get(&destination.location) {
            Some(map) => {
                
                self.data.door = Some(
                    Door {
                        position: destination.position.coords.x as usize + destination.position.coords.y as usize * map.width,
                        tile: map.tile(destination.position.coords).unwrap(),
                        accumulator: 0.0,
                        open: true,
                    }
                );

                self.data.player.character.position.from_destination(destination.position);
                self.data.current = Some(destination.location);
            }
            None => todo!(),
        }
    }

    pub fn do_move(&mut self, delta: f32) -> bool {
        if if let Some(door) = &self.data.door {
            !door.open || door.accumulator == Door::DOOR_MAX
        } else { true } {
            self.data.player.do_move(delta)
        } else {
            false
        }
    }

}

pub fn can_move(move_type: MoveType, code: MovementId) -> bool {
    match move_type {
        MoveType::Swimming => can_swim(code),
        _ => can_walk(code),
    }
}

pub fn can_walk(code: MovementId) -> bool {
    code == 0xC
    // match move_code {
    //     0x0C | 0x00 | 0x04 => true,
    //     _ => false,
    // }
}

pub fn can_swim(code: MovementId) -> bool {
    code == 0x4
}