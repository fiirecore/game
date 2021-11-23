extern crate firecore_pokedex as pokedex;
extern crate firecore_storage as storage;
extern crate firecore_world as worldlib;

use pokedex::{Dex, Initializable, Uninitializable, item::{
        bag::{OwnedBag, SavedBag},
        Item,
    }, moves::Move, pokemon::{
        owned::{OwnedPokemon, SavedPokemon},
        party::Party,
        Pokemon,
    }};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use storage::error::DataError;
use worldlib::{
    character::Character,
    positions::{Coordinate, Direction, Location, LocationId, Position},
    TrainerId,
};

use world::WorldStatus;

mod list;
pub mod world;

pub use list::PlayerSaves;

pub type Name = String;
pub type Worth = u32;

pub type SavedPlayer = Player<SavedPokemon, SavedBag>;
pub type PlayerData<'d> = NewPlayerData<&'d Pokemon, &'d Move, &'d Item>;
pub type NewPlayerData<P, M, I> = Player<OwnedPokemon<P, M, I>, OwnedBag<I>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player<P, B> {
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

    #[serde(default = "Party::default")]
    pub party: Party<P>,

    #[serde(default)]
    pub bag: B,

    #[serde(default)]
    pub worth: Worth,

    /// To - do: move
    #[serde(default)]
    pub world: WorldStatus,
}

impl SavedPlayer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            ..Default::default()
        }
    }

    pub fn save(&self, local: bool) -> Result<(), DataError> {
        use storage::{info, warn};
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

    pub fn init<'d>(
        self,
        random: &mut impl rand::Rng,
        pokedex: &'d dyn Dex<'d, Pokemon, &'d Pokemon>,
        movedex: &'d dyn Dex<'d, Move, &'d Move>,
        itemdex: &'d dyn Dex<'d, Item, &'d Item>,
    ) -> Option<PlayerData<'d>> {

        let mut party = Party::new();

        for p in self.party.into_iter() {
            let p = p.init(random, pokedex, movedex, itemdex)?;
            party.push(p);
        }

        Some(Player {
            should_save: self.should_save,
            id: self.id,
            name: self.name,
            location: self.location,
            character: self.character,
            party,
            bag: self.bag.init(itemdex)?,
            worth: self.worth,
            world: self.world,
        })
    }
}

impl<'d> Uninitializable for PlayerData<'d> {
    type Output = SavedPlayer;

    fn uninit(self) -> Self::Output {
        SavedPlayer {
            should_save: self.should_save,
            id: self.id,
            name: self.name,
            location: self.location,
            character: self.character,
            party: self.party.into_iter().map(Uninitializable::uninit).collect(),
            bag: self.bag.uninit(),
            worth: self.worth,
            world: self.world,
        }
    }
}

impl Default for SavedPlayer {
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
    let t = (storage::time() * 10000.0) as u32;
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
    Character::new(default_position())
}

pub const fn default_position() -> Position {
    Position {
        coords: Coordinate { x: 6, y: 6 },
        direction: Direction::Down,
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