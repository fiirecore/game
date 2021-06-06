use serde::{Deserialize, Serialize};
use super::PlayerSave;

// #[deprecated(note = "to-do: own version of storage::get for this")]
#[derive(Default, Debug, Deserialize, Serialize)]
pub struct PlayerSaves {

    #[serde(skip)]
    pub selected: Option<usize>,
    
    pub saves: Vec<PlayerSave>,

}

impl PlayerSaves {

    pub fn select(&mut self, index: usize) {
        if index <= self.saves.len() {
            self.selected = Some(index);
        }
    }

    pub fn select_new(&mut self, name: &str) {
        let index = self.saves.len();
        self.saves.push(PlayerSave::new(name));
        self.select(index);
    }

    pub fn delete(&mut self, index: usize) -> bool {
        if index <= self.saves.len() {
            self.saves.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get(&self) -> &PlayerSave {
        &self.saves[self.selected.expect("Could not get selected player save!")]
    }

    pub fn get_mut(&mut self) -> &mut PlayerSave {
        &mut self.saves[self.selected.expect("Could not get selected player save!")]
    }

}

#[cfg(feature = "default")]
impl firecore_storage::PersistantData for PlayerSaves {
    fn file_name() -> &'static str {
        "saves"
    }
}