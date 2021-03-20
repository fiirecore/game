use std::path::PathBuf;

use firecore_data::data::PersistantData;
use firecore_data::data::PersistantDataLocation;
use macroquad::prelude::info;
use macroquad::prelude::warn;
use serde::{Deserialize, Serialize};

use super::save::PlayerSave;

const FILE: &str = "saves.ron";

#[derive(Default, Deserialize, Serialize)]
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

    pub fn select_new(&mut self, name: &String) {
        let index = self.saves.len();
        self.saves.push(PlayerSave::new(name));
        self.select(index);
    }

    pub fn get(&self) -> &PlayerSave {
        &self.saves[self.selected.unwrap()]
    }

    pub fn get_mut(&mut self) -> &mut PlayerSave {
        &mut self.saves[self.selected.unwrap()]
    }

    // pub fn load_or_create_new(&self) {
    //     self
    // }

    pub fn name_list(&self) -> Vec<&String> {
        self.saves.iter().map(|data| &data.name).collect()
    }

}

#[async_trait::async_trait(?Send)]
impl PersistantDataLocation for PlayerSaves {

    async fn load_from_file() -> Self {
        Self::load(PathBuf::from(FILE)).await
    }

}

#[async_trait::async_trait(?Send)]
impl PersistantData for PlayerSaves {

    async fn load(path: PathBuf) -> Self {
        info!("Loading player data...");
		match firecore_data::data::read_string(&path).await {
			Ok(data) => {
				match ron::from_str(&data) {
				    Ok(data) => data,
				    Err(err) => {
						warn!("Could not read player data with error {}", err);
                        Self::default()
					}
				}
			},
		    Err(err) => {
				warn!("Could not open player data file at {:?} with error {}", path, err);
                Self::default()
			}
		}
    }

    fn save(&self) {
        firecore_data::data::save_struct(FILE, &self);
    }

}