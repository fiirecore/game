use serde::{Deserialize, Serialize};
use storage::PersistantData;

use crate::engine::{
    input::controls::keyboard::{default_key_map, KeyMap},
    Context,
};
use firecore_storage::{error::DataError, try_load};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    #[serde(default = "default_key_map", skip)]
    pub controls: KeyMap,

    #[serde(default)]
    pub touchscreen: bool,
}

impl Configuration {
    pub async fn load(engine: &mut Context, save_locally: bool) -> Result<Self, DataError> {
        let config = try_load::<Self>(save_locally).await
            .unwrap_or_else(|err| panic!("Could not read configuration with error {}", err));

        crate::engine::log::info!("to - do: allow engine to set controls");
        // engine.controls.keyboard = config.controls.clone();
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
