use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::{character::npc::{NPC, NPCInteract}, script::world::WorldScript};

#[derive(Deserialize, Serialize)]
pub struct WorldMapManagerConstants {
    pub center_npc: NPC,
}

impl Default for WorldMapManagerConstants {
    fn default() -> Self {
        Self {
            center_npc: NPC {
                name: "Joy".to_string(),
                npc_type: "joy".parse().unwrap(),
                character: Default::default(),
                movement: Default::default(),
                origin: Default::default(),
                interact: NPCInteract::Script(WorldScript {
                    identifier: "pkmn_center_heal".parse().unwrap(),
                    location: None,
                    conditions: Vec::new(),
                    original_actions: {
                        let mut actions = VecDeque::new();
                        // actions
                        actions
                    },
                    actions: VecDeque::new(),
                    alive: Default::default(),
                    option: Default::default(),
                    timer: Default::default(),
                }),
                trainer: None,
            },
        }
    }
}