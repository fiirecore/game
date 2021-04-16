use firecore_world_lib::{
    map::{
        World,
        chunk::{
            WorldChunk,
            map::WorldChunkMap,
        },
        warp::WarpDestination,
    },
    character::player::PlayerCharacter,
};

use firecore_game::battle::BattleData;

use firecore_game::macroquad::prelude::warn;

use crate::{GameWorld, TileTextures, NpcTextures, RenderCoords};
use crate::gui::text_window::TextWindow;

impl GameWorld for WorldChunk {

    fn on_start(&mut self, music: bool) {
        self.map.on_start(music);
    }

    fn update(&mut self, delta: f32, player: &mut PlayerCharacter, battle_data: &mut Option<BattleData>, warp: &mut Option<(WarpDestination, bool)>, text_window: &mut TextWindow) {
        self.map.update(delta, player, battle_data, warp, text_window);
    }

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, screen: RenderCoords, border: bool) {
        self.map.render(tile_textures, npc_textures, screen.offset(self.coords), border)
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
        if let Some(chunk) = self.chunk_mut() {
            chunk.on_start(music);
        } else {
            warn!("Could not get current chunk {:?}!", self.current);
        }
    }

    fn update(&mut self, delta: f32, player: &mut PlayerCharacter, battle_data: &mut Option<BattleData>, warp: &mut Option<(WarpDestination, bool)>, text_window: &mut TextWindow) {
        if let Some(chunk) = self.chunk_mut() {
            chunk.update(delta, player, battle_data, warp, text_window);
        }
    }

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, screen: RenderCoords, border: bool) {
        if let Some(chunk) = self.chunk() {
            chunk.render(tile_textures, npc_textures, screen, border);
            for connection in &chunk.connections {
                if let Some(chunk) = self.chunks.get(connection) {
                    chunk.render(tile_textures, npc_textures, screen, false);
                }
            }
        }        
    }

    fn input(&mut self, delta: f32, player: &mut PlayerCharacter) {
        if let Some(chunk) = self.chunk_mut() {
            chunk.input(delta, player);
        }
    }

    fn on_tile(&mut self, battle_data: &mut Option<BattleData>, player: &mut PlayerCharacter) {
        if let Some(chunk) = self.chunk_mut() {
            if chunk.in_bounds(player.position.local.coords) {
                chunk.on_tile(battle_data, player);
            }
        }
    }

}