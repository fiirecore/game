use serde::{Deserialize, Serialize};
use std::{hash::Hash, sync::Arc};

use worldcli::{
    engine::utils::HashMap,
    pokedex::{
        item::Item,
        moves::Move,
        pokemon::Pokemon,
        trainer::{InitTrainer, SavedTrainer},
        Dex,
    },
    worldlib::{script::default::DefaultWorldScriptEngine, state::WorldState},
};

#[derive(Debug)]
pub struct SaveManager<ID: Eq + Hash + Clone> {
    pub current: Option<(ID, InitPlayer)>,
    pub saves: HashMap<ID, SavedPlayer>,
    pub pokedex: Arc<Dex<Pokemon>>,
    pub movedex: Arc<Dex<Move>>,
    pub itemdex: Arc<Dex<Item>>,
}

// mod list;

// pub use list::PlayerSaves;

pub type GameWorldState = WorldState<DefaultWorldScriptEngine>;

pub type SavedPlayer = Player<SavedTrainer>;
pub type InitPlayer = Player<InitTrainer>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player<T> {
    #[serde(default = "default_version")]
    pub version: String,

    #[serde(default)]
    pub world: GameWorldState,

    #[serde(default)]
    pub trainer: T,
}

impl<ID: Eq + Hash + Clone> SaveManager<ID> {

    pub fn new(
        pokedex: Arc<Dex<Pokemon>>,
        movedex: Arc<Dex<Move>>,
        itemdex: Arc<Dex<Item>>,
    ) -> Self {
        Self {
            current: Default::default(),
            saves: Default::default(),
            pokedex,
            movedex,
            itemdex,
        }
    }

    pub fn create(&mut self, id: ID, name: impl Into<String>, rival: impl Into<String>) {
        self.saves.insert(id, Player::new(name, rival));
    }

    pub fn set_current(&mut self, id: ID) -> bool {
        let old = self.current.take();
        if let Some((id, player)) = old {
            self.saves.insert(id, player.uninit());
        }
        let new = self.saves.remove(&id);
        if let Some(new) = new.and_then(|new| new.init(&self.pokedex, &self.movedex, &self.itemdex)) {
            self.current = Some((id, new));
        }
        self.current.is_some()
    }

    pub fn current(&self) -> Option<&InitPlayer> {
        self.current.as_ref().map(|(.., p)| p)
    }

    pub fn current_mut(&mut self) -> Option<&mut InitPlayer> {
        self.current.as_mut().map(|(.., p)| p)
    }

    pub fn save(&mut self) {
        if let Some(current) = self.current.as_ref() {
            self.saves
                .insert(current.0.clone(), current.1.clone().uninit());
        }
        worldcli::engine::log::info!("to - do: save to file");
    }
}

impl<T: Default> Player<T> {
    pub fn new(name: impl Into<String>, rival: impl Into<String>) -> Self {
        Self {
            version: default_version(),
            world: GameWorldState::new(name, rival),
            trainer: Default::default(),
        }
    }
}

impl<T: Default> Default for Player<T> {
    fn default() -> Self {
        Self {
            version: default_version(),
            world: Default::default(),
            trainer: Default::default(),
        }
    }
}

fn default_version() -> String {
    crate::VERSION.to_owned()
}

impl SavedPlayer {
    pub fn init(
        self,
        pokedex: &Dex<Pokemon>,
        movedex: &Dex<Move>,
        itemdex: &Dex<Item>,
    ) -> Option<InitPlayer> {
        Some(InitPlayer {
            version: self.version,
            world: self.world,
            trainer: self
                .trainer
                .init(&mut rand::thread_rng(), pokedex, movedex, itemdex)?,
        })
    }
}

impl InitPlayer {
    pub fn uninit(self) -> SavedPlayer {
        SavedPlayer {
            version: self.version,
            world: self.world,
            trainer: self.trainer.uninit(),
        }
    }
}
