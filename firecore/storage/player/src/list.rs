use crate::PlayerData;

use super::SavedPlayer;
use firecore_pokedex::{item::Item, moves::Move, pokemon::Pokemon, Dex};
use storage::error::DataError;

// #[deprecated(note = "to-do: own version of storage::get for this")]
pub struct PlayerSaves {
    pub selected: Option<PlayerData<'static>>,
    pub list: Vec<SavedPlayer>,
}

#[cfg(target_arch = "wasm32")] 
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WASMSave(SavedPlayer);

#[cfg(target_arch = "wasm32")] 
impl storage::PersistantData for WASMSave {
    fn path() -> &'static str {
        "save"
    }
}

impl PlayerSaves {
    pub async fn load(local: bool) -> Result<Self, DataError> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let dir = storage::directory(local)?.join("saves");
            if !dir.exists() {
                std::fs::create_dir_all(&dir)?;
            }

            let mut list = Vec::new();

            for dir in dir.read_dir()?.flatten() {
                let path = dir.path();
                let p = storage::deserialize::<SavedPlayer>(&std::fs::read_to_string(&path)?)?;
                list.push(p);
            }

            Ok(Self {
                selected: None,
                list,
            })
        }
        #[cfg(target_arch = "wasm32")] 
        {
            use storage::PersistantData;
            
            let player = storage::try_load::<WASMSave>(local).await.unwrap_or_else(|err| {
                storage::warn!("Cannot load save with error {}", err);
                let default = Default::default();
                if let Err(err) = storage::save(&default, local, storage::file_name(WASMSave::path())) {
                    storage::warn!("Cannot save with error {}", err);
                }
                default
            });
            Ok(Self {
                selected: None,
                list: vec![player.0],
            })
        }
    }

    pub fn select(
        &mut self,
        index: usize,
        random: &mut impl rand::Rng,
        pokedex: &'static dyn Dex<'static, Pokemon, &'static Pokemon>,
        movedex: &'static dyn Dex<'static, Move, &'static Move>,
        itemdex: &'static dyn Dex<'static, Item, &'static Item>,
    ) {
        if index < self.list.len() {
            let selected = self.list.remove(index);
            if let Some(selected) = selected.init(random, pokedex, movedex, itemdex) {
                self.selected = Some(selected);
            } else {
                storage::warn!("Cannot select {}! (cant init)", index);
            }
        } else {
            storage::warn!("Cannot select {}! (doesnt exist)", index);
        }
    }

    pub fn select_new(
        &mut self,
        name: &str,
        random: &mut impl rand::Rng,
        pokedex: &'static dyn Dex<'static, Pokemon, &'static Pokemon>,
        movedex: &'static dyn Dex<'static, Move, &'static Move>,
        itemdex: &'static dyn Dex<'static, Item, &'static Item>,
    ) {
        let index = self.list.len();
        self.list.push(SavedPlayer::new(name));
        self.select(index, random, pokedex, movedex, itemdex);
    }

    pub fn select_first_or_default(
        &mut self,
        local: bool,
        random: &mut impl rand::Rng,
        pokedex: &'static dyn Dex<'static, Pokemon, &'static Pokemon>,
        movedex: &'static dyn Dex<'static, Move, &'static Move>,
        itemdex: &'static dyn Dex<'static, Item, &'static Item>,
    ) {
        if self.list.is_empty() {
            let data = SavedPlayer::default();
            if let Err(err) = data.save(local) {
                storage::warn!("Could not save new player file with error {}", err);
            }
            self.list.push(data);
        }
        self.select(0, random, pokedex, movedex, itemdex);
    }

    pub fn delete(&mut self, index: usize) -> bool {
        if index <= self.list.len() {
            self.list.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get(&self) -> &PlayerData<'static> {
        self.selected.as_ref().unwrap()
    }

    pub fn get_mut(&mut self) -> &mut PlayerData<'static> {
        self.selected.as_mut().unwrap()
    }
}

// fn get<S, E: std::error::Error>(save: Option<Option<Result<S, E>>>) -> S {
//     match save {
//         Some(Some(Ok(save))) => save,
//         Some(Some(Err(err))) => panic!("Could not get selected save with error: {}", err),
//         Some(None) => panic!("Could not get selected save as it does not exist."),
//         None => panic!("No save has been selected."),
//     }
// }
