extern crate firecore_dependencies as deps;
extern crate firecore_pokedex as pokedex;
extern crate firecore_storage as storage;
extern crate firecore_world as worldlib;

use pokedex::{item::bag::Bag, pokemon::party::PokemonParty, trainer::TrainerId};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use storage::error::DataError;
use worldlib::{
    character::Character,
    positions::{Coordinate, Direction, Location, LocationId, PixelOffset, Position},
};

use world::WorldStatus;

mod list;
pub mod world;

pub use list::PlayerSaves;

pub type Name = String;
pub type Worth = u32;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerSave {

    #[serde(skip)]
    pub should_save: bool,

    #[serde(default = "default_id")]
    pub id: TrainerId,

    #[serde(default = "default_name")]
    pub name: Name,

    #[serde(default = "default_location")]
    pub location: Location,

    #[serde(default = "default_character")]
    pub character: Character,

    #[serde(default)]
    pub party: PokemonParty,

    #[serde(default)]
    pub bag: Bag,

    #[serde(default)]
    pub worth: Worth,

    #[serde(default)]
    pub world: WorldStatus,
}

impl PlayerSave {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            ..Default::default()
        }
    }

    pub fn save(&self, local: bool) -> Result<(), DataError> {
        use log::{info, warn};
        info!("Saving player data!");
        if let Err(err) = storage::save(
            self,
            local,
            PathBuf::from("saves").join(storage::file_name(&format!("{}-{}", self.name, self.id))),
        ) {
            warn!("Could not save player data with error: {}", err);
        }
        Ok(())
    }
}

impl Default for PlayerSave {
    fn default() -> Self {
        Self {
            should_save: Default::default(),
            id: default_id(),
            name: default_name(),
            party: Default::default(),
            character: default_character(),
            location: default_location(),
            bag: Default::default(),
            worth: 0,
            world: WorldStatus::default(),
        }
    }
}

pub fn default_id() -> TrainerId {
    let t = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or_default();
    let mut str = format!("{}i", t).chars().rev().collect::<String>();
    str.truncate(16);
    str.parse().unwrap_or_else(|err| {
        panic!(
            "Could not parse player id string {} with error {}",
            str, err
        )
    })
}

pub fn default_name() -> String {
    default_name_str().to_owned()
}

pub fn default_name_str() -> &'static str {
    "Red"
}

pub const fn default_location() -> Location {
    Location {
        map: Some(default_map()),
        index: default_index(),
    }
}

pub fn default_character() -> Character {
    Character {
        position: default_position(),
        ..Default::default()
    }
}

pub const fn default_position() -> Position {
    Position {
        coords: Coordinate { x: 6, y: 6 },
        direction: Direction::Down,
        offset: PixelOffset::ZERO,
    }
}

const DEFAULT_MAP: LocationId =
    unsafe { LocationId::new_unchecked(9142636256173598303365790196080u128) };
const DEFAULT_INDEX: LocationId =
    unsafe { LocationId::new_unchecked(132299152847616915686911088u128) };

#[inline]
pub const fn default_map() -> LocationId {
    // To - do: get this from serialized world binary file
    DEFAULT_MAP
}

#[inline]
pub const fn default_index() -> LocationId {
    DEFAULT_INDEX
}
