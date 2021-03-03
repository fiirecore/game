use ahash::AHashMap as HashMap;

use crate::gui::background::Background;
use crate::util::Entity;
use crate::world::player::Player;
use crate::world::script::WorldScript;
use crate::world::script::npc::NPCScript;

#[derive(Default)]
pub struct MapScriptManager {

    pub npc_scripts: Vec<NPCScript>,
    npc_script_clones: HashMap<String, NPCScript>,

    background: Background,

}

impl MapScriptManager {

    pub fn new(npc_scripts: Vec<NPCScript>) -> Self {
        Self {
            npc_scripts,
            ..Default::default()
        }
    }

    pub fn on_tile(&mut self, player: &mut Player) {
        for script in &mut self.npc_scripts {
            if !script.is_alive() && script.location.in_bounds(&player.position.local.coords) {
                match script.run_type {
                    crate::world::script::ScriptRunType::Once => {
                        if !script.has_run() {
                            script.start(player);
                        }
                    }
                    crate::world::script::ScriptRunType::Conditional(ref event) => {
                        if let Some(player_data) = macroquad::prelude::collections::storage::get::<crate::io::data::player::PlayerData>() {
                            if !player_data.world_status.completed_events.contains(event) {
                                if !script.has_run() {
                                    script.start(player);
                                }
                            }
                        }
                    }
                    crate::world::script::ScriptRunType::Always => {
                        if let Some(original) = self.npc_script_clones.get(&script.identifier) {
                            *script = original.clone();
                        } else {
                            self.npc_script_clones.insert(script.identifier.clone(), script.clone());
                        }
                        script.start(player);
                    }
                    crate::world::script::ScriptRunType::AlwaysNoReset => {
                        script.start(player);
                    }
                }
            }
        }
    }

    pub fn update(&mut self, delta: f32, player: &mut Player) {
        for script in &mut self.npc_scripts {            
            if script.is_alive() {
                script.update(delta, player);
            }
        }
    }

    pub fn render(&self, tile_textures: &crate::world::TileTextures, npc_textures: &crate::world::NpcTextures, screen: &crate::world::RenderCoords) {
        for script in &self.npc_scripts {
            if script.is_alive() {
                script.render(tile_textures, npc_textures, &self.background, screen);
            }
        }
    }

}