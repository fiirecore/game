use std::path::PathBuf;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use macroquad::prelude::warn;
use serde::{Serialize, Deserialize};

use crate::util::file::PersistantData;

//use self::controls::ControlConfiguration;

//pub mod controls;

static CONFIGURATION_PATH: &str = "config";
static CONFIGURATION_FILENAME: &str = "config.toml";

#[derive(Serialize, Deserialize)]
pub struct Configuration {
	
	pub window_scale: u8,
	pub save_timer: f32,

	//pub controls: ControlConfiguration,
		
}

impl Configuration {

	fn saved_default() -> Self {
		macroquad::prelude::info!("Creating new configuration file.");
		let default = Configuration::default();
		default.save();
		return default;
	}

	pub async fn load_async_default() -> Self {
		Configuration::load_async(PathBuf::from(CONFIGURATION_PATH).join(CONFIGURATION_FILENAME)).await
	}

	pub async fn load_async(path: PathBuf) -> Self {
		match macroquad::prelude::load_string(path.to_str().expect("Could not get configuration file path as string")).await {
			Ok(data) => Configuration::from_toml(&data),
		    Err(err) => {
				warn!("Could not load configuration file with error {}", err);
				Configuration::saved_default()
			}
		}		
	}

	fn from_toml(data: &str) -> Self {
		match toml::from_str(data) {
			Ok(config) => config,
			Err(err) => {
				warn!("Failed parsing configuration file with error {}", err);
				Configuration::saved_default()
			}
		}
	}
	
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
			window_scale: 3,
			save_timer: 60.0,
			//controls: ControlConfiguration::default(),
		}
    }
}

impl crate::util::file::PersistantDataLocation for Configuration {

	fn load_from_file() -> Self {
		match crate::util::file::read_to_string_noasync(PathBuf::from(CONFIGURATION_PATH).join(CONFIGURATION_FILENAME)) {
			Some(data) => Configuration::from_toml(&data),
		    None => {
				warn!("Could not load configuration file!");
				Configuration::saved_default()
			}
		}
	}

}

impl PersistantData for Configuration {

	// fn load(path: PathBuf) -> Self {
	// 	return match crate::util::file::read_to_string_noasync(path) {
	// 		Some(content) => {
	// 			Configuration::from_toml(&content)
	// 		}
	// 		None => {
	// 			warn!("Failed reading configuration file to string with error");
	// 			Configuration::saved_default()
	// 		}
	// 	};
	// }

	fn save(&self) {
		if cfg!(not(target_arch = "wasm32")) {
			let path = PathBuf::from(CONFIGURATION_PATH);

			if !path.exists() {
				match std::fs::create_dir_all(&path) {
					Ok(()) => {}
					Err(err) => {
						warn!("Could not create folder for configuration directory with error {}", err);
					}
				}
			}
	
			if let Ok(file) = File::create(path.join(CONFIGURATION_FILENAME)) {
				let mut writer = BufWriter::new(file);
	
				match toml::to_string_pretty(&self) {
					Ok(encoded) => {
						if let Err(err) = writer.write(encoded.as_bytes()) {
							warn!("Failed to write configuration: {}", err);
						}
					},
					Err(err) => {
						warn!("Failed to create/open configuration file: {}", err);
					}
				}
			}
		}
	}

	// fn reload(&mut self) {
	// 	info!("Attempting to reload configuration...");
	// 	*self = Configuration::load_from_file();
	// 	info!("Reloaded configuration!");
	// }
	
}