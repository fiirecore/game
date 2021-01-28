use std::path::PathBuf;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::io::Write;
use serde::{Serialize, Deserialize};

use crate::util::file::PersistantData;
use crate::util::file::PersistantDataLocation;

//use self::controls::ControlConfiguration;

//pub mod controls;

static CONFIGURATION_PATH: &str = "config";
static CONFIGURATION_FILENAME: &str = "config.toml";

#[derive(Serialize, Deserialize)]
pub struct Configuration {
	
	pub window_scale: u8,

	//pub controls: ControlConfiguration,
		
}

impl Configuration {

	pub fn get() -> parking_lot::MutexGuard<'static, Self> {
		crate::CONFIGURATION.lock()
	}

	pub fn saved_default() -> Self {
		let default = Configuration::default();
		default.save();
		return default;
	}
	
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
			window_scale: 3,
			//controls: ControlConfiguration::default(),
		}
    }
}

impl PersistantDataLocation for Configuration {

	fn load_from_file() -> Self {
		return Configuration::load(Path::new(CONFIGURATION_PATH).join(CONFIGURATION_FILENAME));
	}

}

impl PersistantData for Configuration {

	fn load(path: PathBuf) -> Self {
		return match crate::util::file::read_to_string_noasync(path) {
			Some(content) => {
				match toml::from_str(content.as_str()) {
					Ok(config) => config,
					Err(err) => {
						println!("Failed parsing configuration file with error {}", err);
						Configuration::saved_default()
					}
				}
			}
			None => {
				println!("Failed reading configuration file to string with error");
				Configuration::saved_default()
			}
		};
	}

	fn save(&self) {
		#[cfg(not(target_arch = "wasm32"))] {
			let path = PathBuf::from(CONFIGURATION_PATH);

			if !path.exists() {
				for folder in path.ancestors() {
					if !folder.exists() {
						std::fs::create_dir(folder).expect("Could not create configuration directory!"); // use format! here
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
	}

	// fn reload(&mut self) {
	// 	info!("Attempting to reload configuration...");
	// 	*self = Configuration::load_from_file();
	// 	info!("Reloaded configuration!");
	// }
	
}