extern crate firecore_input as input;
extern crate firecore_storage as storage;

use serde::{Deserialize, Serialize};

use input::keyboard::{KeyMap, default_key_map};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {

	#[serde(default = "default_key_map")]
	pub controls: KeyMap,	

	#[serde(default)]
	pub touchscreen: bool,

}

impl Default for Configuration {
    fn default() -> Self {
        Self {
			controls: default_key_map(),
			touchscreen: false,
		}
    }
}

impl storage::PersistantData for Configuration {
    fn file_name() -> &'static str {
        "config"
    }
}

impl storage::Reloadable for Configuration {
    fn on_reload(&self) {
        use storage::macroquad::prelude::info;
        info!("Running configuration reload tasks...");
        input::keyboard::load(self.controls.clone());
        input::touchscreen::touchscreen(self.touchscreen);
        info!("Finished configuration reload tasks!");
    }
}