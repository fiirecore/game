extern crate firecore_world as worldlib;

use crate::pokedex::{
    item::{
        bag::{OwnedBag, SavedBag},
        Item,
    },
    moves::Move,
    pokemon::{
        owned::{OwnedPokemon, SavedPokemon},
        party::Party,
        Pokemon,
    },
    Initializable, Uninitializable,
};
use firecore_battle_gui::pokedex::engine::log::info;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, path::PathBuf};
use crate::storage::error::DataError;
use worldlib::{
    character::{player::PlayerCharacter, Character},
    map::manager::state::{default_location, default_position},
    TrainerId,
};

mod list;

pub use list::PlayerSaves;

pub type Name = String;

pub type PlayerData = OwnedPlayer<&'static Pokemon, &'static Move, &'static Item>;

pub type GamePokemon = OwnedPokemon<&'static Pokemon, &'static Move, &'static Item>;
pub type GameParty = Party<GamePokemon>;

pub type GameBag = OwnedBag<&'static Item>;

pub type SavedPlayer = Player<SavedPokemon, SavedBag>;
pub type OwnedPlayer<P, M, I> = Player<OwnedPokemon<P, M, I>, OwnedBag<I>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player<P, B> {

    #[serde(default = "default_id")]
    pub id: TrainerId,

    #[serde(default = "default_name")]
    pub name: Name,

    #[serde(default = "default_character")]
    pub character: PlayerCharacter,

    #[serde(default = "Party::default")]
    pub party: Party<P>,

    #[serde(default)]
    pub bag: B,

}

impl SavedPlayer {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    pub fn save(&self, local: bool) -> Result<(), DataError> {
        use crate::storage::{info, warn};
        info!("Saving player data!");
        if let Err(err) = crate::storage::save(
            self,
            local,
            PathBuf::from("saves").join(crate::storage::file_name(&format!("{}-{}", self.name, self.id))),
        ) {
            warn!("Could not save player data with error: {}", err);
        }
        Ok(())
    }

    pub fn init(
        self,
        random: &mut impl rand::Rng,
        // pokedex: &'d dyn Dex<'d, Pokemon, &'d Pokemon>,
        // movedex: &'d dyn Dex<'d, Move, &'d Move>,
        // itemdex: &'d dyn Dex<'d, Item, &'d Item>,
    ) -> Option<PlayerData> {
        let mut party = Party::new();

        let itemdex = crate::itemdex();

        for p in self.party.into_iter() {
            let p = p.init(random, crate::pokedex(), crate::movedex(), itemdex)?;
            party.push(p);
        }

        Some(Player {
            id: self.id,
            name: self.name,
            character: self.character,
            party,
            bag: self.bag.init(itemdex)?,
        })
    }
}

impl<P: Deref<Target = Pokemon>, M: Deref<Target = Move>, I: Deref<Target = Item>> Uninitializable
    for OwnedPlayer<P, M, I>
{
    type Output = SavedPlayer;

    fn uninit(self) -> Self::Output {
        SavedPlayer {
            id: self.id,
            name: self.name,
            character: self.character,
            party: self
                .party
                .into_iter()
                .map(Uninitializable::uninit)
                .collect(),
            bag: self.bag.uninit(),
        }
    }
}

impl Default for SavedPlayer {
    fn default() -> Self {
        Self {
            id: default_id(),
            name: default_name(),
            party: Default::default(),
            character: default_character(),
            bag: Default::default(),
        }
    }
}

pub fn default_id() -> TrainerId {
    let t = (crate::storage::time() * 10000.0) as u32;
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

pub fn default_character() -> PlayerCharacter {
    PlayerCharacter {
        location: default_location(),
        character: Character::new(default_position()),
        input_frozen: false,
        ignore: false,
        world: Default::default(),
    }
}
