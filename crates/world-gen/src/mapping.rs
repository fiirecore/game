use std::ops::Deref;

use firecore_world_builder::{
    builder::structs::BuilderLocation,
    world::{
        audio::{SoundId, SoundVariant},
        character::CharacterGroupId,
        map::{object::ObjectType, PaletteId, TileId, TransitionId},
        positions::{Direction, Location},
    },
};
use hashbrown::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NameMappings {
    pub map: MapMappings,
    pub palettes: PaletteMappings,
    pub music: HashMap<String, tinystr::TinyStr16>,
    pub npcs: NpcMappings,
    pub objects: ObjectMappings,
    pub audio: AudioMappings,
}

#[derive(Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MapMappings {
    pub id: IdMappings,
    pub name: HashMap<String, String>,
    pub transition: HashMap<String, TransitionId>,
}

#[derive(Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PaletteMappings {
    pub primary: HashMap<String, PaletteId>,
    pub secondary: HashMap<String, PaletteId>,
    pub sizes: HashMap<PaletteId, TileId>,
}

#[derive(Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NpcMappings {
    pub groups: HashMap<String, CharacterGroupId>,
    pub movement: HashMap<String, (bool, HashSet<Direction>)>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct ObjectMappings {
    pub objects: HashMap<String, ObjectType>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct AudioMappings {
    pub sounds: HashMap<String, (SoundId, SoundVariant)>,
}

#[derive(Default, Deserialize, Serialize)]
#[serde(transparent, deny_unknown_fields)]
pub struct IdMappingsFrom {
    pub inner: HashMap<String, BuilderLocation>,
}

#[derive(Default, Deserialize, Serialize)]
#[serde(from = "IdMappingsFrom")]
pub struct IdMappings {
    inner: HashMap<String, Location>,
}

impl Deref for IdMappings {
    type Target = HashMap<String, Location>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<IdMappingsFrom> for IdMappings {
    fn from(mappings: IdMappingsFrom) -> Self {
        Self {
            inner: mappings
                .inner
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}