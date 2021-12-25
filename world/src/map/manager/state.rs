use std::{cell::Cell, rc::Rc};

use serde::{Deserialize, Serialize};

use hashbrown::{HashMap, HashSet};

use crate::{
    character::npc::{trainer::BadgeId, Npc, NpcId},
    map::{battle::TrainerEntry, warp::WarpDestination},
    positions::{Location, LocationId, Position},
    script::ScriptId,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorldMapState {
    #[serde(default)]
    pub battle: WorldBattleState,
    #[serde(default)]
    pub npc: WorldNpcData,
    #[serde(default)]
    pub warp: Option<WarpDestination>,
    #[serde(default)]
    pub scripts: WorldGlobalScriptData,
    #[serde(default)]
    pub wild: WorldGlobalWildData,
    #[serde(default)]
    pub heal: Option<(Location, Position)>,
    #[serde(default)]
    pub badges: HashSet<BadgeId>,
    #[serde(skip)]
    pub polling: Option<Rc<Cell<bool>>>,
    #[serde(default)]
    pub debug_draw: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorldBattleState {
    pub battled: HashMap<Location, HashSet<NpcId>>,
    pub battling: Option<TrainerEntry>,
}
impl WorldBattleState {
    pub fn insert(&mut self, location: &Location, npc: NpcId) {
        if let Some(battled) = self.battled.get_mut(location) {
            battled.insert(npc);
        } else {
            let mut battled = HashSet::with_capacity(1);
            battled.insert(npc);
            self.battled.insert(*location, battled);
        }
    }

    pub fn battled(&self, map: &Location, npc: &NpcId) -> bool {
        self.battled
            .get(map)
            .map(|battled| battled.contains(npc))
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorldGlobalScriptData {
    pub executed: HashSet<ScriptId>,
    // pub actions: Vec<WorldAction>,
    pub npcs: HashMap<NpcId, (Location, Npc)>,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct WorldNpcData {
    pub active: Option<NpcId>,
    pub timer: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldGlobalWildData {
    pub encounters: bool,
}

impl Default for WorldGlobalWildData {
    fn default() -> Self {
        Self { encounters: true }
    }
}

pub const fn default_heal_loc() -> (Location, Position) {
    (default_location(), default_position())
}

pub const fn default_location() -> Location {
    Location {
        map: Some(default_map()),
        index: default_index(),
    }
}

pub const fn default_position() -> Position {
    Position {
        coords: crate::positions::Coordinate { x: 6, y: 6 },
        direction: crate::positions::Direction::Down,
        elevation: None,
    }
}

const DEFAULT_MAP: LocationId = unsafe { LocationId::new_unchecked(127978959561072u128) };
const DEFAULT_INDEX: LocationId =
    unsafe { LocationId::new_unchecked(132299152847616915686911088u128) };

#[inline]
pub const fn default_map() -> LocationId {
    // To - do: get this from serialized world binary file
    DEFAULT_MAP
}

#[inline]
pub const fn default_index() -> LocationId {
    DEFAULT_INDEX
}
