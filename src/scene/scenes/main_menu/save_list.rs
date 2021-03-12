use macroquad::prelude::info;
use macroquad::prelude::warn;
use serde::{Deserialize, Serialize};
use ahash::AHashSet as HashSet;

#[derive(Default, Deserialize, Serialize)]
pub struct SaveList {
    pub players: HashSet<String>,
}

const FILE: &str = "saves.ron";

impl SaveList {

    pub fn get() -> Self {
        match Self::load_noasync() {
		    Some(contents) => {
				let result: Result<SaveList, ron::Error> = ron::from_str(&contents);
				match result {
				    Ok(save_list) => {
						info!("Successfully read save list!");
						save_list
					}
				    Err(err) => {
						warn!("Could not decode save list! Returning a new save list. Error: {}", err);
                        default()
					}
				}
			}
		    None => {
				warn!("Save list could not be found, creating a new one!");
				default()
			}
		}
    }

	fn load_noasync() -> Option<String> {
		#[cfg(not(target_arch = "wasm32"))]
		{
			match firecore_data::get_save_dir() {
				Ok(dir) => return Some(crate::util::file::noasync::read_to_string_noasync(dir.join(FILE))?),
				Err(_) => return None,
			}        
		}
		#[cfg(target_arch = "wasm32")]
		{
			match path.as_ref().file_name() {
				Some(fname) => return Some(miniquad_cookie::get_cookie("saves")),
				None => return None,
			}
		}
	}

    pub fn append(name: &str) {
        let mut list = Self::get();
        list.players.insert(name.to_owned());
        firecore_data::data::save_struct(FILE, &list);
    }

}

fn default() -> SaveList {
    let default = SaveList::default();
    firecore_data::data::save_struct(FILE, &default);
    default
}