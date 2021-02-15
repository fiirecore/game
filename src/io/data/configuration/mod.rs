use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use async_trait::async_trait;
use macroquad::prelude::KeyCode;
use macroquad::prelude::info;
use macroquad::prelude::warn;
use serde::{Serialize, Deserialize};
use crate::util::file::PersistantData;
use crate::util::file::PersistantDataLocation;
use ahash::{AHashMap as HashMap, AHashSet as HashSet};
use crate::util::input::Control;

static CONFIGURATION_PATH: &str = "config";
static CONFIGURATION_FILENAME: &str = "config.json";

#[derive(Serialize, Deserialize)]
pub struct Configuration {
	pub controls: HashMap<Control, HashSet<KeyCode>>,	
}

impl Configuration {

	pub fn on_reload(&self) {
		info!("Running configuration reload tasks...");
		crate::util::input::keyboard::reload_with_config(self);
		info!("Finished configuration reload tasks!");
	}

	fn saved_default() -> Self {
		macroquad::prelude::info!("Creating new configuration file.");
		let default = Configuration::default();
		default.save();
		return default;
	}

	fn default_path() -> PathBuf {
		PathBuf::from(CONFIGURATION_PATH).join(CONFIGURATION_FILENAME)
	}

	fn from_string(data: &str) -> Self {
		match serde_json::from_str(data) {
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
			controls: crate::util::input::keyboard::default(),
		}
    }
}

#[async_trait(?Send)]
impl PersistantDataLocation for Configuration {

	async fn load_from_file() -> Self {
		Configuration::load(Self::default_path()).await
	}

}

#[async_trait(?Send)]
impl PersistantData for Configuration {

	async fn load(path: PathBuf) -> Self {
		return match crate::util::file::read_to_string(path).await {
			Ok(content) => Configuration::from_string(&content),
			Err(err) => {
				warn!("Failed reading configuration file to string with error {}", err);
				Configuration::saved_default()
			}
		};
	}

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
	
			if let Ok(mut file) = File::create(path.join(CONFIGURATION_FILENAME)) {
				match serde_json::to_string_pretty(&self) {
					Ok(encoded) => {
						if let Err(err) = file.write(encoded.as_bytes()) {
							warn!("Failed to write to configuration file: {}", err);
						}
					},
					Err(err) => {
						warn!("Failed to parse configuration into a string with error: {}", err);
					}
				}
			}
		}
	}

	async fn reload(&mut self) {
		info!("Attempting to reload configuration...");
		*self = Configuration::load_from_file().await;
		info!("Reloaded configuration!");
		self.on_reload();
	}
	
}