use std::path::PathBuf;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::io::Write;
use std::sync::MutexGuard;

use log::info;
use serde::{Serialize, Deserialize};

use crate::util::file::PersistantData;
use crate::util::file::PersistantDataLocation;

pub mod controls;

static CONFIGURATION_PATH: &str = "config";
static CONFIGURATION_FILENAME: &str = "config.toml";

#[derive(Serialize, Deserialize)]
pub struct Configuration {
	
	pub scale: u8,
		
}

impl Configuration {
	
	pub fn new() -> Self {
		Configuration::load_from_file()
	}

	pub fn get() -> MutexGuard<'static, Self> {
		crate::CONFIGURATION.lock().expect("Could not get configuration!")
	}
	
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
			scale: 3,
		}
    }
}

impl PersistantDataLocation for Configuration {

	fn load_from_file() -> Self {
		return Configuration::load(Path::new(CONFIGURATION_PATH).join(CONFIGURATION_FILENAME));
	}

}

impl PersistantData for Configuration {

	fn load<P: AsRef<Path>>(path: P) -> Self {
		let path = path.as_ref();

		match std::fs::read_to_string(path) {
			Ok(content) => {
				match toml::from_str(content.as_str()) {
					Ok(config) => {
						return config;
					}
					Err(err) => {
						println!("Failed parsing configuration file with error {}", err);
						let default = Configuration::default();
						default.save();
						return default;
					}
				}
			}
			Err(err) => {
				println!("Failed reading configuration file to string with error {}", err);
				let default = Configuration::default();
				default.save();
				return default;
			}
		}

	}

	fn save(&self) {

		let path = PathBuf::from(CONFIGURATION_PATH);

		if !path.exists() {
			for folder in path.ancestors() {
				if !folder.exists() {
					std::fs::create_dir(folder).expect("Could not create configuration directory!");
				}
			}
		}

		if let Ok(file) = File::create(path.join(CONFIGURATION_FILENAME)) {
			let mut writer = BufWriter::new(file);

			match toml::to_string_pretty(&self) {
				Ok(encoded) => {
					if let Err(err) = writer.write(encoded.as_bytes()) {
						println!("Failed to write configuration: {}", err);
					}
				},
				Err(err) => {
					println!("Failed to create/open configuration file: {}", err);
				}
			}
		}
	}

	fn reload(&mut self) {
		*self = Configuration::new();
		info!("Reloaded configuration!");
		*crate::WINDOW_SCALE.lock().unwrap() = self.scale;
	}
	
}