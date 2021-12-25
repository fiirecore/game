use crate::{
    map::warp::WarpId,
    positions::{Location, Position},
};
use serde::{Deserialize, Serialize};

use crate::{character::npc::NpcId, map::warp::WarpDestination};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldAction {
    /// Checks if
    TrainerBattleSingle(NpcId),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerWarp {
    Id(WarpId),
    Dest(WarpDestination),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NpcWarp {
    Id(WarpId),
    Dest(Location, Position),
}
