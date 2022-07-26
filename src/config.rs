use serde::{Deserialize, Serialize};

use crate::engine::{
    controls::keyboard::{default_key_map, KeyMap},
    Plugins,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    #[serde(skip, default = "default_key_map")]
    pub controls: KeyMap,
}

impl Configuration {
    pub fn load(&self, plugins: &mut Plugins) {
        crate::engine::controls::keyboard::set_key_map(plugins, self.controls);
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            controls: default_key_map(),
        }
    }
}
