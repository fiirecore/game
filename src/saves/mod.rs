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
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use worldlib::{
    character::{player::PlayerCharacter, Character},
    map::manager::state::{default_location, default_position},
};

// mod list;

// pub use list::PlayerSaves;

pub type PlayerData = OwnedPlayer<&'static Pokemon, &'static Move, &'static Item>;

pub type GamePokemon = OwnedPokemon<&'static Pokemon, &'static Move, &'static Item>;
pub type GameParty = Party<GamePokemon>;

pub type GameBag = OwnedBag<&'static Item>;

pub type SavedPlayer = Player<SavedPokemon, SavedBag>;
pub type OwnedPlayer<P, M, I> = Player<OwnedPokemon<P, M, I>, OwnedBag<I>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player<P, B> {
    #[serde(default = "default_id")]
    pub id: u64,

    #[serde(default = "default_character")]
    pub character: PlayerCharacter,

    #[serde(default = "Party::default")]
    pub party: Party<P>,

    #[serde(default)]
    pub bag: B,
}

impl SavedPlayer {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            character: PlayerCharacter {
                rival: "Gary".to_owned(),
                character: Character {
                    name: name.into(),
                    position: default_position(),
                    ..Default::default()
                },
                ..default_character()
            },
            ..Default::default()
        }
    }

    // pub fn save(&self) -> Result<(), DataError> {
    //     info!("Saving player data!");
    //     if let Err(err) = crate::storage::save(
    //         self,
    //         crate::PUBLISHER,
    //         crate::APPLICATION,
    //         PathBuf::from("saves").join(&format!("{}-{}.ron", self.character.name, self.id)),
    //     ) {
    //         warn!("Could not save player data with error: {}", err);
    //     }
    //     Ok(())
    // }

    pub fn init(
        self,
        random: &mut impl rand::Rng,
        // pokedex: &'d dyn Dex<'d, Pokemon, &'d Pokemon>,
        // movedex: &'d dyn Dex<'d, Move, &'d Move>,
        // itemdex: &'d dyn Dex<'d, Item, &'d Item>,
    ) -> Option<PlayerData> {
        let mut party = Party::new();

        let itemdex = crate::dex::itemdex();

        for p in self.party.into_iter() {
            let p = p.init(
                random,
                crate::dex::pokedex(),
                crate::dex::movedex(),
                itemdex,
            )?;
            party.push(p);
        }

        Some(Player {
            id: self.id,
            character: self.character,
            party,
            bag: self.bag.init(itemdex)?,
        })
    }
}

impl storage::PersistantData for SavedPlayer {
    fn path() -> &'static str {
        "save"
    }
}

impl<P: Deref<Target = Pokemon>, M: Deref<Target = Move>, I: Deref<Target = Item>> Uninitializable
    for OwnedPlayer<P, M, I>
{
    type Output = SavedPlayer;

    fn uninit(self) -> Self::Output {
        SavedPlayer {
            id: self.id,
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
            party: Default::default(),
            character: default_character(),
            bag: Default::default(),
        }
    }
}

pub fn default_id() -> u64 {
    // let t = crate::engine::utils::seed();
    // let mut str = format!("{}i", t).chars().rev().collect::<String>();
    // str.truncate(16);
    // str.parse().unwrap_or_else(|err| {
    //     panic!(
    //         "Could not parse player id string {} with error {}",
    //         str, err
    //     )
    // })
    crate::engine::utils::seed()
}

pub fn default_name() -> &'static str {
    "Red"
}

pub fn default_rival() -> &'static str {
    "Gary"
}

pub fn default_character() -> PlayerCharacter {
    PlayerCharacter {
        location: default_location(),
        character: Character::new(default_name(), default_position()),
        input_frozen: false,
        ignore: false,
        world: Default::default(),
        rival: default_rival().to_owned(),
    }
}
