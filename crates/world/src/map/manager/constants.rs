use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::{character::npc::{Npc, NpcInteract}, script::world::WorldScript};

#[derive(Deserialize, Serialize)]
pub struct WorldMapManagerConstants {
    pub center_npc: Npc,
}

impl Default for WorldMapManagerConstants {
    fn default() -> Self {
        Self {
            center_npc: Npc {
                name: "Joy".to_string(),
                npc_type: "joy".parse().unwrap(),
                character: Default::default(),
                movement: Default::default(),
                origin: Default::default(),
                interact: NpcInteract::Script(WorldScript {
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