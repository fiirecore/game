use serde::{Deserialize, Serialize};

use deps::str::TinyStr16;

pub type TrainerId = TinyStr16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainerData {
    pub npc_type: TrainerId,
    pub prefix: String,
    pub name: String,
}