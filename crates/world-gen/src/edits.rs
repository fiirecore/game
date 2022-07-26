use dashmap::DashMap;
use firecore_world::{
        character::npc::NpcId,
        map::{
            warp::{WarpDestination, WarpEntry},
            WorldMap,
        },
        positions::{Coordinate, CoordinateInt, Destination, Location},
    };
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use crate::structs::{BuilderArea, BuilderLocation};

#[derive(Deserialize, Serialize)]
pub struct Edits {
    pub maps: HashMap<BuilderLocation, MapEdits>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct MapEdits {
    #[serde(default)]
    pub npcs: Vec<NpcEdits>,
    #[serde(default)]
    pub warps: Vec<WarpEdits>,
}

#[derive(Deserialize, Serialize)]
pub enum NpcEdits {
    Remove(NpcId),
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum WarpEdits {
    Add(BuilderArea, EditDestination),
    Remove(CoordinateInt, CoordinateInt),
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct EditDestination {
    pub location: BuilderLocation,
    pub position: Destination,
}

impl Edits {
    // pub fn load() -> Self {
    //     let path = std::path::Path::new("./edits.ron");

    //     match std::fs::read_to_string(path) {
    //         Ok(data) => match ron::from_str(&data) {
    //             Ok(edits) => edits,
    //             Err(err) => panic!("Cannot deserialize edits with error {}", err),
    //         },
    //         Err(err) => {
    //             panic!("Could not load edits file with error {}", err)
    //         }
    //     }
    // }

    pub fn process(self, maps: &DashMap<Location, WorldMap>) {
        let edits = self
            .maps
            .into_iter()
            .map(|(k, v)| (k.into(), v))
            .collect::<HashMap<Location, MapEdits>>();
        for mut map in maps.iter_mut() {
            if let Some(edit) = edits.get(map.key()) {
                for npc in &edit.npcs {
                    match npc {
                        NpcEdits::Remove(id) => {
                            map.npcs.remove(id);
                        }
                    }
                }
                for warp in edit.warps.iter().copied() {
                    match warp {
                        WarpEdits::Add(area, dest) => map.warps.push(WarpEntry {
                            area: area.into(),
                            destination: WarpDestination {
                                location: dest.location.into(),
                                position: dest.position,
                            },
                        }),
                        WarpEdits::Remove(x, y) => {
                            if let Some(index) = map
                                .warps
                                .iter()
                                .position(|w| w.area.contains(&Coordinate { x, y }))
                            {
                                map.warps.remove(index);
                            }
                        }
                    }
                }
            }
        }
    }
}
