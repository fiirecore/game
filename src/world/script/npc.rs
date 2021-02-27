use macroquad::prelude::warn;
use serde::Deserialize;
use crate::audio::music::Music;
use crate::gui::Focus;
use crate::gui::GuiComponent;
use crate::gui::background::Background;
use crate::gui::dynamic_text::DynamicText;
use crate::util::Completable;
use crate::util::Coordinate;
use crate::util::Entity;
use crate::util::Input;
use crate::util::Position;
use crate::world::NpcTextures;
use crate::world::RenderCoords;
use crate::world::TileTextures;
use crate::world::npc::NPC;
use crate::world::player::Player;

use super::ScriptRunType;
use super::WorldScript;

#[derive(Clone, Deserialize)]
pub struct NPCScript {

    #[serde(skip)]
    pub has_ran: bool,
    #[serde(skip)]
    pub alive: bool,

    pub identifier: String,
    
    pub run_type: ScriptRunType,

    pub location: Coordinate,

    pub npc: ScriptNPC,
    pub end_position: Position,

}

impl NPCScript {

    pub fn new(file: std::path::PathBuf) -> Option<Self> {
        match crate::io::get_file_as_string(file) {
            Ok(content) => {
                match serde_json::from_str(&content) {
                    Ok(script) => {
                        return Some(script);
                    }
                    Err(err) => {
                        warn!("Could not parse NPC script with error {}", err);
                        return None;
                    }
                }
            }
            Err(err) => {
                warn!("Could not open NPC script with error {}", err);
                return None;
            }
        }
    }

}

impl WorldScript for NPCScript {

    fn start(&mut self, player: &mut Player) {
        player.freeze();
        self.npc.npc.walk_to(&self.end_position.coords);
        self.spawn();
    }

    fn update(&mut self, delta: f32, player: &mut Player) {
        if self.npc.npc.should_move() {
            self.npc.npc.do_move(delta);
        } else if !self.npc.message.is_finished() {
            if !self.npc.message.is_alive() {
                self.npc.message.spawn();
                self.npc.message.focus();
                self.npc.npc.position.direction = self.end_position.direction;
                player.position.local.direction = self.end_position.direction.inverse();
            }
            self.npc.message.input(delta);
            self.npc.message.update(delta);
        } else {
            crate::util::battle_data::trainer_battle(&self.npc.npc);
            self.finish(player);
        }
    }

    fn render(&self, _tile_textures: &TileTextures, npc_textures: &NpcTextures, background: &Background, screen: &RenderCoords) {
        self.npc.npc.render(npc_textures, screen);
        if self.npc.message.is_alive() {
            background.render();
        }
        self.npc.message.render();
    }

    fn finish(&mut self, player: &mut Player) {
        player.frozen = false;
        self.despawn();
    }

    fn has_run(&self) -> bool {
        self.has_ran
    }

}

impl Entity for NPCScript {

    fn spawn(&mut self) {
        self.alive = true;
        self.has_ran = true;
    }

    fn despawn(&mut self) {
        self.alive = false;
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}

#[derive(Clone, Deserialize)]
pub struct ScriptNPC {
    
    pub npc: NPC,
    pub message: DynamicText,
    pub music: Option<Music>,

}