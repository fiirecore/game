use std::fs::{create_dir_all, read_to_string};

use crate::PlayerData;

use super::SavedPlayer;
use firecore_pokedex::{Dex, item::Item, moves::Move, pokemon::Pokemon};
use storage::error::DataError;

// #[deprecated(note = "to-do: own version of storage::get for this")]
pub struct PlayerSaves<'d> {
    pub selected: Option<PlayerData<'d>>,
    pub list: Vec<SavedPlayer>,
}

impl<'d> PlayerSaves<'d> {
    pub fn load(local: bool) -> Result<Self, DataError> {
        let dir = storage::directory(local)?.join("saves");
        if !dir.exists() {
            create_dir_all(&dir)?;
        }

        let mut list = Vec::new();

        for dir in dir.read_dir()?.flatten() {
            let path = dir.path();
            let p = storage::deserialize::<SavedPlayer>(&read_to_string(&path).map_err(DataError::IOError)?)
                .map_err(DataError::Deserialize)?;
                list.push(p);
        }


        Ok(Self {
            selected: None,
            list,
        })
    }

    pub fn select(
        &mut self,
        index: usize,
        random: &mut impl rand::Rng,
        pokedex: &'d dyn Dex<'d, Pokemon, &'d Pokemon>,
        movedex: &'d dyn Dex<'d, Move, &'d Move>,
        itemdex: &'d dyn Dex<'d, Item, &'d Item>,
    ) {
        if index < self.list.len() {
            let selected = self.list.remove(index);
            if let Some(selected) = selected.init(random, pokedex, movedex, itemdex) {
                self.selected = Some(selected);
            } else {
                log::warn!("Cannot select {}! (cant init)", index);
            }
        } else {
            log::warn!("Cannot select {}! (doesnt exist)", index);
        }
    }

    pub fn select_new(&mut self, name: &str,
        random: &mut impl rand::Rng,
        pokedex: &'d dyn Dex<'d, Pokemon, &'d Pokemon>,
        movedex: &'d dyn Dex<'d, Move, &'d Move>,
        itemdex: &'d dyn Dex<'d, Item, &'d Item>,) {
        let index = self.list.len();
        self.list.push(SavedPlayer::new(name));
        self.select(index, random, pokedex, movedex, itemdex);
    }

    pub fn select_first_or_default(&mut self, local: bool,
        random: &mut impl rand::Rng,
        pokedex: &'d dyn Dex<'d, Pokemon, &'d Pokemon>,
        movedex: &'d dyn Dex<'d, Move, &'d Move>,
        itemdex: &'d dyn Dex<'d, Item, &'d Item>,) {
        // if self.list.iter().flatten().count() == 0 {
            let data = SavedPlayer::default();
            // if let Err(err) = data.save(local) {
            //     log::warn!("Could not save new player file with error {}", err);
            // }
            self.list.push(data);
        // }
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

    pub fn get(&self) -> &PlayerData<'d> {
        self.selected.as_ref().unwrap()
    }

    pub fn get_mut(&mut self) -> &mut PlayerData<'d> {
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
