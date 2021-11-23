use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::{
    character::{
        npc::{trainer::TrainerDisable, Npc, NpcId},
        player::PlayerCharacter,
    },
    map::warp::WarpDestination,
    positions::Location,
    script::world::WorldAction,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorldMapData {
    pub location: Option<Location>,
    pub player: PlayerCharacter,
    #[serde(default)]
    pub npc: WorldNpcData,
    #[serde(default)]
    pub warp: Option<WarpDestination>,
    #[serde(default)]
    pub script: WorldGlobalScriptData,
    #[serde(default)]
    pub wild: WorldGlobalWildData,
    #[serde(default)]
    pub battling: Option<TrainerEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainerEntry {
    pub map: Location,
    pub id: NpcId,
    pub disable_others: TrainerDisable,
}

pub type TrainerEntryRef<'a> = &'a mut Option<TrainerEntry>;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorldGlobalScriptData {
    pub actions: Vec<WorldAction>,
    pub npcs: HashMap<NpcId, (Location, Npc)>,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct WorldNpcData {
    pub active: Option<NpcId>,
    pub timer: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WorldGlobalWildData {
    pub encounters: bool,
}

impl Default for WorldGlobalWildData {
    fn default() -> Self {
        Self { encounters: true }
    }
}
