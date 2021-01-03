//use directories_next::ProjectDirs;
use crate::util::traits::PersistantDataLocation;
use crate::util::traits::PersistantData;
use std::path::PathBuf;
use std::ffi::OsString;
use std::fs::File;
use std::env::current_exe;
use std::io::BufWriter;
use std::io::Read;
use std::path::Path;
use std::io::Write;

use log::info;
use serde_derive::{Serialize, Deserialize};

static CONFIGURATION_FILENAME: &str = "config.toml";

pub struct Configuration {
	
	pub name: String,
	pub authors: String,
	pub version: String,
	
	pub width: usize,
	pub height: usize,
	pub scale: usize,
	
	pub asset_folder: PathBuf,
	pub saves_folder: PathBuf,
	
	pub window_clear_color: [f32; 3],
	
	pub key_hold_timer: usize,
		
}

impl Configuration {
	
	pub fn load() -> Configuration {
		Configuration::get_variables(ConfigurationInToml::load_from_file())
	}
	
	fn get_variables(config: ConfigurationInToml) -> Configuration {
		
		let dimensions = config.dimensions.unwrap();
		
		let folders = config.folders.unwrap();
		
		Configuration {
			
			// Immutable variables
			
			name: "Pokemon Game".to_string(),
			authors: "Rhys Holloway".to_string(),
			version: "INDEV".to_string(),	
			//seed: rand::thread_rng().gen_range(0, 255),
			
			window_clear_color: [0f32; 3],
			
			// Externally mutable variables
			
			width: dimensions.width.unwrap(),
			height: dimensions.height.unwrap(),
			scale: dimensions.scale.unwrap(),
			
			asset_folder: PathBuf::from(folders.asset_folder.unwrap()),
			saves_folder: PathBuf::from(folders.saves_folder.unwrap()),
			
			key_hold_timer: config.controls_misc.as_ref().unwrap().key_hold_timer.unwrap(),
			
		}
		
	}

	pub fn reload(&mut self) {
		*self = Configuration::load();
		info!("Reloaded configuration!");
		//self.name = conf.name;
	}
	
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigurationInToml {
	
	dimensions: Option<Dimensions>,
	folders: Option<Folders>,
	controls_misc: Option<ControlsMisc>,
	
}

#[derive(Debug, Serialize, Deserialize)]
struct Dimensions {
	
	width: Option<usize>,
	height: Option<usize>,
	scale: Option<usize>,
	
}

#[derive(Debug, Serialize, Deserialize)]
struct Folders {
	
	asset_folder: Option<String>,
	saves_folder: Option<String>,
	
}

#[derive(Debug, Serialize, Deserialize)]
struct ControlsMisc {
	
	key_hold_timer: Option<usize>,
	
}

impl ConfigurationInToml {
	
	pub fn defaults() -> ConfigurationInToml {
		
		//let pb = ProjectDirs::from("net", "Rhys Holloway", "Pokemon").unwrap().data_dir().to_path_buf();
		//pb.pop();
		//let dir = pb.as_path();
		
		let assets;
		let saves;
		
		//if cfg!(debug_assertions) {
			assets = "assets";
			saves = "saves";
		//} else {
		//	assets = dir.clone().join("assets").to_str().unwrap();
		//	saves = dir.clone().join("saves").to_str().unwrap();
		//}
		
		ConfigurationInToml {
			
			dimensions: Some(Dimensions {
				width: Some(240),
				height: Some(160),
				scale: Some(3),
			}),
			
			folders: Some(Folders {
				asset_folder: Some(assets.to_string()),
				saves_folder: Some(saves.to_string()),
			}),
			
			controls_misc: Some(ControlsMisc {
				key_hold_timer: Some(200),
			}),			
			
		}
		
	}
	
}

impl PersistantDataLocation for ConfigurationInToml {

	fn load_from_file() -> Self {
		let path: OsString;

//		if cfg!(debug_assertions) {
			let exe_path = current_exe();
			if exe_path.is_err() {
				return ConfigurationInToml::defaults();
			}
			let mut exe_path = exe_path.unwrap();
			exe_path.pop();
			path = exe_path.join(Path::new(CONFIGURATION_FILENAME)).into_os_string();
//		} else {
//			let dir = ProjectDirs::from("net", "Rhys Holloway", "Pokemon").unwrap();
//			path = dir.config_dir().join(CONFIGURATION_FILENAME).into_os_string();
//		}
		return ConfigurationInToml::load(path);
	}

}

impl PersistantData for ConfigurationInToml {

	fn load<P>(path: P) -> Self where P: AsRef<Path> {
		let path = path.as_ref();

		let mut content = String::new();
		
		match File::open(&path) {
			
			Ok(mut file) => {
				file.read_to_string(&mut content).unwrap();
			},
			
			Err(error) => {
				println!("Failed");
				println!("Error opening configuration file at {}", path.to_str().unwrap());
				println!("{}", error);
				println!("Creating a new configuration file.");
                let default = ConfigurationInToml::defaults();
                default.save();
     			return default;
			}
			
		}

		let toml_result: Result<ConfigurationInToml, toml::de::Error> = toml::from_str(content.as_str());
		
		match toml_result {
			Ok(toml) => {
				return toml;
			}
			Err(e) => {
				println!("Could not read configuration file with error {}", e);
				println!("Creating a new configuration file.");
				let default = ConfigurationInToml::defaults();
                default.save();
     			return default;
			}
		}

	}

	fn save(&self) {
		
//		if cfg!(debug_assertions) {
			//let exe_path = current_exe();
			//if exe_path.is_err() {
			//	println!("WARNING: Failed to save settings: can't find exe path.");
			//	return;
			//}
			//path = exe_path.unwrap().join("config");

			let path = PathBuf::from("config");

			if !path.exists() {
				std::fs::create_dir("config").expect("Could not create configuration directory!");
			}
			
//		} else {
//			path = ProjectDirs::from("net", "Rhys Holloway", "Pokemon").unwrap().config_dir().to_path_buf().into_os_string();
//		}
		
		let file = File::create(Path::new(&path).join(CONFIGURATION_FILENAME)).unwrap();
		let mut writer = BufWriter::new(file);

		match toml::to_string_pretty(&self) {
			Ok(encoded) => {
				if let Err(e) = writer.write(encoded.as_bytes()) {
					println!("WARNING: Failed to save settings: {}", e);
				}
			},
			Err(e) => {
				println!("WARNING: Failed to save settings: {}", e);
			}
		}

	}
	
}