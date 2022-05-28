use hashbrown::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

use pokedex::pokemon::owned::SavedPokemon;

use audio::{SoundId, SoundVariant};
use text::MessageState;

use crate::{
    character::npc::{group::MessageColor, trainer::BadgeId, Npc, NpcId},
    map::{battle::BattleEntry, warp::WarpDestination, MusicId},
    positions::{Coordinate, Location, Position},
    script::{ScriptEnvironment, ScriptId, Variable, VariableName},
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorldState {
    #[serde(default)]
    pub events: Vec<WorldEvent>,
    #[serde(default)]
    pub objects: HashMap<Location, Vec<Coordinate>>,
    #[serde(default)]
    pub battle: GlobalBattleState,
    #[serde(default)]
    pub message: Option<MessageState<MessageColor>>,
    #[serde(default)]
    pub npc: GlobalNpcData,
    #[serde(default)]
    pub warp: Option<WarpDestination>,
    #[serde(default)]
    pub scripts: GlobalScriptData,
    #[serde(default)]
    pub wild: GlobalWildData,
    #[serde(default)]
    pub heal: Option<(Location, Position)>,
    #[serde(default)]
    pub badges: HashSet<BadgeId>,
    #[serde(default)]
    pub debug_draw: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldEvent {
    PlayMusic(MusicId),
    PlaySound(SoundId, SoundVariant),
    BeginWarpTransition(Coordinate),
    PlayerJump,
    BreakObject(Coordinate),

    /// Should freeze player and start battle
    // Battle(BattleEntry),
    OnTile,
    // Command(PlayerActions),
}

pub type Battled = HashSet<NpcId>;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalBattleState {
    pub battled: HashMap<Location, Battled>,
    pub battling: Option<BattleEntry<SavedPokemon>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GlobalScriptData {
    pub executed: HashSet<ScriptId>,
    pub npcs: HashMap<NpcId, (Location, Npc)>,
    pub flags: HashSet<VariableName>,
    pub variables: HashMap<VariableName, Variable>,
    pub environment: ScriptEnvironment,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct GlobalNpcData {
    pub timer: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalWildData {
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

impl GlobalBattleState {
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

impl Default for GlobalWildData {
    fn default() -> Self {
        Self { encounters: true }
    }
}
