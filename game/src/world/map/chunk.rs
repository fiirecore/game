use crate::{
    battle_glue::BattleEntryRef,
    macroquad::prelude::warn
};

use worldlib::{
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

use crate::world::{
    GameWorld,
    WorldTextures,
    RenderCoords,
    gui::TextWindow,
};

impl GameWorld for WorldChunk {

    fn on_start(&mut self, music: bool) {
        self.map.on_start(music);
    }

    fn update(&mut self, delta: f32, player: &mut PlayerCharacter, battle: BattleEntryRef, warp: &mut Option<WarpDestination>, text_window: &mut TextWindow) {
        self.map.update(delta, player, battle, warp, text_window);
    }

    fn render(&self, textures: &WorldTextures, screen: RenderCoords, border: bool) {
        self.map.render(textures, screen.offset(self.coords), border)
    }

    fn on_tile(&mut self, battle: BattleEntryRef, player: &mut PlayerCharacter) {
        self.map.on_tile(battle, player)
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

    fn update(&mut self, delta: f32, player: &mut PlayerCharacter, battle: BattleEntryRef, warp: &mut Option<WarpDestination>, text_window: &mut TextWindow) {
        if let Some(chunk) = self.chunk_mut() {
            chunk.update(delta, player, battle, warp, text_window);
        }
    }

    fn render(&self, textures: &WorldTextures, screen: RenderCoords, border: bool) {
        if let Some(chunk) = self.chunk() {
            chunk.render(textures, screen, border);
            for connection in &chunk.connections {
                if let Some(chunk) = self.chunks.get(connection) {
                    chunk.render(textures, screen, false);
                }
            }
        }        
    }

    fn on_tile(&mut self, battle: BattleEntryRef, player: &mut PlayerCharacter) {
        if let Some(chunk) = self.chunk_mut() {
            if chunk.in_bounds(player.character.position.coords) {
                chunk.on_tile(battle, player);
            }
        }
    }

}