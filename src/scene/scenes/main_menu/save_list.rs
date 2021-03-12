use macroquad::miniquad::fs::Error;
use macroquad::prelude::info;
use macroquad::prelude::warn;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use ahash::AHashSet as HashSet;

lazy_static::lazy_static! {
	static ref EPIC_FAIL: Mutex<Option<Result<Vec<u8>, Error>>> = Mutex::new(None);
}

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
				Ok(dir) => {
					macroquad::miniquad::fs::load_file(&*dir.join(FILE).to_string_lossy(), |bytes| {
						*EPIC_FAIL.lock() = Some(bytes);
					});
					if let Some(result) = EPIC_FAIL.lock().take() { 
						match result {
						    Ok(bytes) => {
								String::from_utf8(bytes).ok()
							}
						    Err(err) => {
								warn!("{}", err);
								None
							}
						}
					} else {
						None
					}
				}
				Err(err) => {
					warn!("Could not get save directory with error {}", err);
					None
				},
			}
			      
		}
		#[cfg(target_arch = "wasm32")]
		{
			let saves = miniquad_cookie::get_cookie("saves");
			return if saves.is_empty() {
				None
			} else {
				Some(saves)
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