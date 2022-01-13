use serde::{Deserialize, Serialize};

use crate::{
    character::{npc::Npcs, player::PlayerCharacter},
    positions::{Coordinate, CoordinateInt, Direction, Location},
};

use self::{
    chunk::WorldChunk,
    movement::MapMovementResult,
    object::{ItemObject, Items, MapObject, Objects, SignObject, Signs},
    warp::{WarpDestination, Warps},
    wild::WildEntries,
};

pub mod manager;

pub mod movement;
pub mod tile;

pub mod chunk;

pub mod object;
pub mod warp;
pub mod wild;

pub mod battle;

pub type TileId = tile::TileId;
pub type WorldTile = tile::WorldTile;
pub type PaletteId = u8;
pub type Palettes = [PaletteId; 2];
pub type MapSize = usize;
pub type Border = tile::Border;
pub type MovementId = movement::MovementId;
pub type MusicId = tinystr::TinyStr16;
pub type TransitionId = tinystr::TinyStr8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldMap {
    pub id: Location,

    pub name: String,
    pub music: MusicId,

    pub width: CoordinateInt,
    pub height: CoordinateInt,

    pub palettes: Palettes,

    pub tiles: Vec<WorldTile>,
    pub movements: Vec<MovementId>,

    pub border: Border, //Border, // border blocks

    pub chunk: Option<WorldChunk>,

    // Map objects
    pub warps: Warps,

    pub wild: Option<WildEntries>,

    pub npcs: Npcs,
    pub objects: Objects,
    pub items: Items,
    pub signs: Signs,

    // pub objects: HashMap<u8, MapObject>,
    // pub scripts: Vec<WorldScript>,
    #[serde(default)]
    pub settings: WorldMapSettings,
    // pub mart: Option<mart::Pokemart>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorldMapSettings {
    #[serde(default)]
    /// To - do: rename to "fly"
    pub fly_position: Option<Coordinate>,
    #[serde(default)]
    pub brightness: Brightness,
    #[serde(default = "WorldMapSettings::default_transition")]
    pub transition: TransitionId,
}

impl WorldMap {
    pub fn in_bounds(&self, coords: Coordinate) -> bool {
        !(coords.x.is_negative()
            || coords.x >= self.width
            || coords.y.is_negative()
            || coords.y >= self.height)
    }

    pub fn tile(&self, coords: Coordinate) -> Option<WorldTile> {
        self.in_bounds(coords)
            .then(|| self.tiles[coords.x as usize + coords.y as usize * self.width as usize])
    }

    pub fn local_movement(
        &self,
        coords: Coordinate,
        player: &PlayerCharacter,
    ) -> Option<MovementId> {
        self.in_bounds(coords)
            .then(|| self.unbounded_movement(coords, player))
            .flatten()
    }

    pub fn unbounded_movement(
        &self,
        coords: Coordinate,
        player: &PlayerCharacter,
    ) -> Option<MovementId> {
        self.movements
            .get(coords.x as usize + coords.y as usize * self.width as usize)
            .map(|code| {
                // Iterators
                let npcs = self.npcs.values().map(|npc| npc.character.position.coords);
                let objects = self
                    .objects
                    .keys()
                    .filter(|coordinate| !player.world.contains_object(&self.id, coordinate))
                    .copied();
                let items = self
                    .items
                    .iter()
                    .filter(|(coordinate, object)| {
                        !object.hidden || !player.world.contains_object(&self.id, coordinate)
                    })
                    .map(|(c, ..)| *c);
                // find used locations
                match npcs.chain(objects).chain(items).any(|c| c == coords) {
                    true => 1,
                    false => *code,
                }
            })
    }

    pub fn chunk_movement(
        &self,
        coords: Coordinate,
        player: &PlayerCharacter,
    ) -> MapMovementResult {
        if let Some(chunk) = self.chunk.as_ref() {
            if coords.x.is_negative() {
                return chunk
                    .connections
                    .get_key_value(&Direction::Left)
                    .map(|(d, c)| (d, coords.y, c.as_ref()))
                    .into();
            }

            if coords.x >= self.width {
                return chunk
                    .connections
                    .get_key_value(&Direction::Right)
                    .map(|(d, c)| (d, coords.y, c.as_ref()))
                    .into();
            }

            if coords.y.is_negative() {
                return chunk
                    .connections
                    .get_key_value(&Direction::Up)
                    .map(|(d, c)| (d, coords.x, c.as_ref()))
                    .into();
            }

            if coords.y >= self.height {
                return chunk
                    .connections
                    .get_key_value(&Direction::Down)
                    .map(|(d, c)| (d, coords.x, c.as_ref()))
                    .into();
            }
        } else if !self.in_bounds(coords) {
            return MapMovementResult::NONE;
        }
        self.unbounded_movement(coords, player).into()
    }

    pub fn warp_at(&self, coordinate: &Coordinate) -> Option<&WarpDestination> {
        self.warps
            .iter()
            .find(|warp| warp.area.contains(coordinate))
            .map(|entry| &entry.destination)
    }

    pub fn object_at(&self, coordinate: &Coordinate) -> Option<&MapObject> {
        self.objects.get(&coordinate)
    }

    pub fn sign_at(&self, coordinate: &Coordinate) -> Option<&SignObject> {
        self.signs.get(coordinate)
    }

    pub fn item_at(&self, coordinate: &Coordinate) -> Option<&ItemObject> {
        self.items.get(coordinate)
    }

    pub fn contains(&self, location: &Location) -> bool {
        &self.id == location
            || self
                .chunk
                .as_ref()
                .map(|chunk| {
                    chunk
                        .connections
                        .values()
                        .flatten()
                        .any(|connection| &connection.0 == location)
                })
                .unwrap_or_default()
    }
}

impl WorldMapSettings {
    pub fn default_transition() -> TransitionId {
        "default".parse().unwrap()
    }
}

impl Default for WorldMapSettings {
    fn default() -> Self {
        Self {
            fly_position: Default::default(),
            brightness: Default::default(),
            transition: Self::default_transition(),
        }
    }
}

/// To - do: move
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum Brightness {
    Day,
    Night,
    // FlashNeeded,
}

// #[deprecated]
// #[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
// pub enum MapIcon {
//     City(u8, u8),
//     Route(u8, u8, u8, u8),
// }

impl Default for Brightness {
    fn default() -> Self {
        Self::Day
    }
}
