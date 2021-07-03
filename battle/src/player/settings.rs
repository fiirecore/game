use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct PlayerSettings {
    pub gains_exp: bool,
    pub request_party: bool,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            gains_exp: true,
            request_party: false,
        }
    }
}