use serde::{Deserialize, Serialize};

use hashbrown::{HashMap, HashSet};

use crate::{
    character::npc::{trainer::BadgeId, Npc, NpcId},
    events::Wait,
    map::{battle::BattleEntry, warp::WarpDestination},
    positions::{Coordinate, Location, Position},
    script::ScriptId,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorldState {
    #[serde(default)]
    pub objects: HashMap<Location, Vec<Coordinate>>,
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
    pub polling: Option<Wait>,
    #[serde(default)]
    pub debug_draw: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorldBattleState {
    pub battled: HashMap<Location, HashSet<NpcId>>,
    pub battling: Option<BattleEntry>,
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

impl WorldState {
    pub fn contains_object(&self, location: &Location, coordinate: &Coordinate) -> bool {
        self.objects
            .get(location)
            .map(|coords| coords.contains(coordinate))
            .unwrap_or_default()
    }

    pub fn insert_object(&mut self, location: &Location, coordinate: Coordinate) {
        if !self.objects.contains_key(location) {
            self.objects.insert(*location, Default::default());
        }
        let objects = self.objects.get_mut(location).unwrap();
        objects.push(coordinate)
    }
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

impl Default for WorldGlobalWildData {
    fn default() -> Self {
        Self { encounters: true }
    }
}
