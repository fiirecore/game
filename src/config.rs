use serde::{Deserialize, Serialize};

use crate::input::keyboard::{KeyMap, default_key_map, load};
use crate::log::info;

pub static mut CONFIGURATION: Option<Configuration> = None;

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
    fn path() -> &'static str {
        "config"
    }
}

impl storage::Reloadable for Configuration {
    fn on_reload(&self) {
        info!("Running configuration reload tasks...");
        load(self.controls.clone());
        // input::touchscreen::touchscreen(self.touchscreen);
        info!("Finished configuration reload tasks!");
    }
}