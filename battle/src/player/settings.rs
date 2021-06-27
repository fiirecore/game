use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct PlayerSettings {
    pub gains_exp: bool,
}