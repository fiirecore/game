use firecore_world::map::World;
use firecore_world::map::chunk::WorldChunk;
use firecore_world::map::chunk::map::WorldChunkMap;
use macroquad::prelude::warn;

use crate::battle::data::BattleData;
use crate::world::NPCTypes;
use crate::world::{GameWorld, TileTextures, NpcTextures, GuiTextures, RenderCoords};
use crate::world::gui::text_window::TextWindow;
use firecore_world::character::player::PlayerCharacter;

impl GameWorld for WorldChunk {

    fn on_start(&mut self, music: bool) {
        self.map.on_start(music);
    }

    fn update(&mut self, delta: f32, player: &mut PlayerCharacter, battle_data: &mut Option<BattleData>, text_window: &mut TextWindow, npc_types: &NPCTypes) {
        self.map.update(delta, player, battle_data, text_window, npc_types);
    }

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, npc_types: &NPCTypes, gui_textures: &GuiTextures, screen: RenderCoords, border: bool) {
        self.map.render(tile_textures, npc_textures, npc_types, gui_textures, screen.offset(self.coords), border)
    }

    fn on_tile(&mut self, battle_data: &mut Option<BattleData>, player: &mut PlayerCharacter) {
        self.map.on_tile(battle_data, player)
    }

    fn input(&mut self, delta: f32, player: &mut PlayerCharacter) {
        self.map.input(delta, player)
    }

}

impl GameWorld for WorldChunkMap {

    fn on_start(&mut self, music: bool) {
        self.current_chunk_mut().on_start(music);
    }

    fn update(&mut self, delta: f32, player: &mut PlayerCharacter, battle_data: &mut Option<BattleData>, text_window: &mut TextWindow, npc_types: &NPCTypes) {
        self.current_chunk_mut().update(delta, player, battle_data, text_window, npc_types);
    }

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, npc_types: &NPCTypes, gui_textures: &GuiTextures, screen: RenderCoords, border: bool) {
        match self.chunks.get(&self.current_chunk) {
            Some(chunk) => {
                chunk.render(tile_textures, npc_textures, npc_types, gui_textures, screen, border);
                for connection in &chunk.connections {
                    self.chunks.get(connection).expect("Could not get connected chunk").render(tile_textures, npc_textures, npc_types, gui_textures, screen, false);
                }
            }
            None => {
                warn!("Could not get current chunk to render");
            }
        }
        
    }

    fn input(&mut self, delta: f32, player: &mut PlayerCharacter) {
        self.current_chunk_mut().input(delta, player)
    }

    fn on_tile(&mut self, battle_data: &mut Option<BattleData>, player: &mut PlayerCharacter) {
        let current_chunk = self.current_chunk_mut();
        if current_chunk.in_bounds(player.position.local.coords) {
            current_chunk.on_tile(battle_data, player);
        }
    }

}