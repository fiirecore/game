use serde::{Deserialize, Serialize};
use storage::PersistantData;

use crate::engine::{
    input::controls::keyboard::{default_key_map, KeyMap},
    Context,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    #[serde(default = "default_key_map")]
    pub controls: KeyMap,
}

impl Configuration {

    pub fn load(&self, ctx: &mut Context) {
        crate::engine::input::controls::keyboard::set_key_map(ctx, self.controls);
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
