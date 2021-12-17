use super::SavedPlayer;
use crate::pokedex::Dex;
use crate::storage::{self, error::DataError};

// #[deprecated(note = "to-do: own version of storage::get for this")]
pub struct PlayerSaves {
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

            Ok(Self { list })
        }
        #[cfg(target_arch = "wasm32")]
        {
            use storage::PersistantData;

            let player = storage::try_load::<WASMSave>(local)
                .await
                .unwrap_or_else(|err| {
                    storage::warn!("Cannot load save with error {}", err);
                    let default = Default::default();
                    if let Err(err) =
                        storage::save(&default, local, storage::file_name(WASMSave::path()))
                    {
                        storage::warn!("Cannot save with error {}", err);
                    }
                    default
                });
            Ok(Self {
                list: vec![player.0],
            })
        }
    }

    pub fn delete(&mut self, index: usize) -> bool {
        if index <= self.list.len() {
            self.list.remove(index);
            true
        } else {
            false
        }
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
