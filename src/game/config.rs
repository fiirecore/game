use storage::PersistantData;
use serde::{Deserialize, Serialize};

use engine::{input::keyboard::{KeyMap, default_key_map}, tetra::Result, EngineContext};
use firecore_storage::try_load;

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {

	#[serde(default = "default_key_map")]
	pub controls: KeyMap,	

	#[serde(default)]
	pub touchscreen: bool,

}

impl Configuration {

	pub fn load(engine: &mut EngineContext, save_locally: bool) -> Result<Self> {
		let config = try_load::<Self>(save_locally)
        .unwrap_or_else(|err| panic!("Could not read configuration with error {}", err));

		engine.controls.keyboard = config.controls.clone();
		// engine.controls.controller = config.controls.controller.clone();

		Ok(config)
	}

}

impl Default for Configuration {
    fn default() -> Self {
        Self {
			controls: default_key_map(),
			touchscreen: false,
		}
    }
}

impl PersistantData for Configuration {
    fn path() -> &'static str {
        "config"
    }
}