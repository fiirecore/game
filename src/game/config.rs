use serde::{Deserialize, Serialize};
use storage::PersistantData;

use crate::engine::{
    controls::keyboard::{default_key_map, KeyMap},
    EngineContext,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    #[serde(default = "default_key_map")]
    pub controls: KeyMap,
}

impl Configuration {
    pub fn load(&self, eng: &mut EngineContext) {
        crate::engine::controls::keyboard::set_key_map(eng, self.controls);
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            controls: default_key_map(),
        }
    }
}

impl PersistantData for Configuration {
    fn path() -> &'static str {
        "config"
    }
}
