use std::fs::{read_to_string, create_dir_all};

use super::PlayerSave;
use storage::error::DataError;

// #[deprecated(note = "to-do: own version of storage::get for this")]
#[derive(Debug)]
pub struct PlayerSaves {

    pub selected: Option<usize>,
    
    pub saves: Vec<Result<PlayerSave, DataError>>,

}

impl PlayerSaves {

    pub fn load() -> Result<Self, DataError> {
        let dir = storage::directory()?.join("saves");
        if !dir.exists() {
            create_dir_all(&dir)?;
        }
        Ok(Self {
            selected: None,
            saves: dir.read_dir()?.flatten().map(|dir| {
                let path = dir.path();
                storage::deserialize::<PlayerSave>(&read_to_string(&path)?).map_err(DataError::Deserialize)
            }).collect()
        })
        
    }

    pub fn select(&mut self, index: usize) {
        if index <= self.saves.len() {
            self.selected = Some(index);
        }
    }

    pub fn select_new(&mut self, name: &str) {
        let index = self.saves.len();
        self.saves.push(Ok(PlayerSave::new(name)));
        self.select(index);
    }

    pub fn select_first_or_default(&mut self) {
        if self.saves.is_empty() {
            let data = PlayerSave::default();
            if let Err(err) = data.save() {
                deps::log::warn!("Could not save new player file with error {}", err);
            }
            self.saves.push(Ok(data));
        }
        self.select(0);
    }

    pub fn delete(&mut self, index: usize) -> bool {
        if index <= self.saves.len() {
            if let Ok(..) = self.saves.remove(index) {}
            true
        } else {
            false
        }
    }

    pub fn get(&self) -> &PlayerSave {
        get(self.selected.map(|index| self.saves.get(index).map(|r| r.as_ref())))
    }

    pub fn get_mut(&mut self) -> &mut PlayerSave {
        get(self.try_get_mut())
    }

    pub fn try_get_mut(&mut self) -> Option<Option<Result<&mut PlayerSave, &DataError>>> {
        self.selected.map(move |index| self.saves.get_mut(index).map(|r| match r {
            Ok(s) => Ok(s),
            Err(e) => Err(&*e),
        }))
    }

}

fn get<S, E: std::error::Error>(save: Option<Option<Result<S, E>>>) -> S {
    match save {
        Some(Some(Ok(save))) => save,
        Some(Some(Err(err))) => panic!("Could not get selected save with error: {}", err),
        Some(None) => panic!("Could not get selected save as it does not exist."),
        None => panic!("No save has been selected."),
    }
}