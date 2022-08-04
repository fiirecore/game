use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use audio::{SoundId, SoundVariant};
use text::MessageStates;

use crate::{
    character::{npc::NpcId, player::PlayerCharacter, CharacterState},
    map::{
        data::{WorldMapData},
        movement::Elevation,
        warp::WarpDestination,
        MusicId,
    },
    message::{MessageColor, MessageTheme},
    positions::{Coordinate, Location, Spot},
};

pub type MapMessage = MessageStates<MessageColor, MessageTheme>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapState {
    /// The currently loaded map
    #[serde(default)]
    pub location: Location,

    /// Player state
    pub player: PlayerCharacter,

    /// Object states
    #[serde(default)]
    pub entities: HashMap<Location, EntityStates>,

    #[serde(default)]
    pub events: Vec<MapEvent>,
    #[serde(default)]
    pub places: GlobalPlacesData,
    #[serde(default)]
    pub npc: GlobalNpcData,
    #[serde(default)]
    pub warp: Option<WarpDestination>,

    #[serde(default)]
    pub message: MapMessage,

    #[serde(default)]
    pub debug_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapEvent {
    PlayMusic(Option<MusicId>),
    PlaySound(SoundId, SoundVariant),
    BeginWarpTransition(Coordinate),
    PlayerJump,
    // Battle(BattleEntry),
    // Command(PlayerActions),
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EntityStates {
    #[serde(skip)]
    pub npcs: HashMap<NpcId, CharacterState>,
    // #[serde(default)]
    // pub objects: HashMap<ObjectId, ObjectEntityState>,
    // #[serde(default)]
    // pub items: HashMap<ObjectId, ItemEntityState>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GlobalNpcData {
    pub timer: f32,
    #[deprecated]
    pub results: Vec<(NpcId, crate::character::DoMoveResult)>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GlobalPlacesData {
    #[serde(default)]
    pub heal: Option<Spot>,
    #[serde(default)]
    pub cave: Option<Spot>,
}

impl MapState {
    pub fn new(name: impl Into<String>, rival: impl Into<String>) -> Self {
        Self {
            player: PlayerCharacter::new(name, rival),
            ..Default::default()
        }
    }

    /// to - do: make better object resetting
    pub fn update_objects(&mut self, data: &WorldMapData) {
        self.entities.retain(|loc, _| loc == &self.location);
        if let Some(map) = data.maps.get(&self.location) {
            if !self.entities.contains_key(&self.location) {
                self.entities.insert(self.location, Default::default());
            }

            let objects = self.entities.get_mut(&self.location).unwrap();

            for (id, npc) in map.npcs.iter() {
                objects.npcs.insert(
                    *id,
                    CharacterState {
                        group: npc.group,
                        position: npc.origin,
                        ..Default::default()
                    },
                );
            }

            // for (id, object) in map.objects.iter() {
            //     objects.objects.insert(
            //         *id,
            //         ObjectEntityState {
            //             entity: *object,
            //             removed: false,
            //         },
            //     );
            // }
        }
    }

    #[deprecated]
    pub fn warp(
        location: &mut Location,
        character: &mut CharacterState,
        destination: WarpDestination,
    ) {
        character.position.from_destination(destination.position);
        character.actions.clear();
        *location = destination.location;
        character.position.elevation = Elevation(0);
    }
}

// impl MapObjectState {
//     pub fn contains_object(&self, location: &Location, coordinate: &Coordinate) -> bool {
//         self.entities
//             .get(location)
//             .map(|coords| coords.contains(coordinate))
//             .unwrap_or_default()
//     }

//     pub fn insert_object(&mut self, location: &Location, coordinate: Coordinate) {
//         if !self.entities.contains_key(location) {
//             self.entities.insert(*location, Default::default());
//         }
//         let objects = self.entities.get_mut(location).unwrap();
//         objects.push(coordinate)
//     }
// }

impl Default for MapState {
    fn default() -> Self {
        Self {
            location: Default::default(),
            player: Default::default(),
            events: Default::default(),
            places: Default::default(),
            npc: Default::default(),
            entities: Default::default(),
            warp: Default::default(),
            message: Default::default(),
            debug_mode: Default::default(),
        }
    }
}
