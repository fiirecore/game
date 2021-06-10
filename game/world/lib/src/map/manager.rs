use serde::{Deserialize, Serialize};
use deps::hash::HashMap;
use util::{Direction, Coordinate, Location};
use crate::{
    character::{
        MoveType,
        player::PlayerCharacter,
        npc::{NpcId, trainer::TrainerDisable},
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
    StartWarpOnTile(TileId, Coordinate),
}

pub type Maps = HashMap<Location, WorldMap>;

#[derive(Deserialize, Serialize)]
pub struct WorldMapManager {

    pub maps: Maps,
    #[serde(skip)]
    pub data: WorldMapManagerData,

}

#[derive(Default)]
pub struct WorldMapManagerData {

    // pub constants: constants::WorldMapManagerConstants,

    // #[serde(skip)]
    pub current: Option<Location>,

    // #[serde(skip)]
    pub player: PlayerCharacter,

    // #[serde(skip)]
    pub warp: Option<WarpDestination>,

    // #[serde(skip)]
    pub battling: Option<TrainerEntry>,

}

pub struct TrainerEntry {
    pub map: Location,
    pub id: NpcId,
    pub disable_others: TrainerDisable,
}

pub type TrainerEntryRef<'a> = &'a mut Option<TrainerEntry>;

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

    pub fn player(&mut self) -> &mut PlayerCharacter {
        &mut self.data.player
    }

    pub fn try_move(&mut self, direction: Direction, delta: f32) -> Option<TryMoveResult> { // return chunk update

        let mut result = None;

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

                        result = Some(TryMoveResult::StartWarpOnTile(self.get().map(|m| m.tile(coords)).flatten().unwrap_or_default(), coords));

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

        #[deprecated(note = "add jumping")]
        fn one_way_tile(tile_id: TileId) -> bool {
            matches!(tile_id, 135 | 176 | 177 | 143 | 151 | 184 | 185 | 192 | 193 | 217 | 1234)
        }

        let walk = self.tile(coords).map(|tile_id| match direction {
            Direction::Up => false,
            Direction::Down => one_way_tile(tile_id),
            Direction::Left => tile_id == 133,
            Direction::Right => tile_id == 134,
        }).unwrap_or_default();
        
        let allow = warp || walk;

        let allow = if !allow {
            // checks if player is inside a solid tile or outside of map, lets them move if true
            // also checks if player is on a one way tile
            if self.tile(self.data.player.character.position.coords).map(one_way_tile).unwrap_or(false) {
                false
            } else {
                self.movement(self.data.player.character.position.coords).map(|code| !can_move(self.data.player.character.move_type, code)).unwrap_or(true)
            }
        } else {
            allow
        };            

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

        result
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

    pub fn warp(&mut self, destination: WarpDestination) -> bool {
        match self.maps.contains_key(&destination.location) {
            true => {
                self.data.player.character.position.from_destination(destination.position);
                self.data.current = Some(destination.location);

                true
            }
            false => todo!(),
        }
    }

    pub fn do_move(&mut self, delta: f32) -> bool {
        self.data.player.do_move(delta)
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
}

pub fn can_swim(code: MovementId) -> bool {
    code == 0x4
}