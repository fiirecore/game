use crate::{
    battle_glue::BattleEntryRef,
    tetra::Context,
    log::warn,
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

    fn on_start(&mut self, ctx: &mut Context, music: bool) {
        self.map.on_start(ctx, music);
    }

    fn update(&mut self, ctx: &mut Context, delta: f32, player: &mut PlayerCharacter, battle: BattleEntryRef, warp: &mut Option<WarpDestination>, text_window: &mut TextWindow) {
        self.map.update(ctx, delta, player, battle, warp, text_window);
    }

    fn draw(&self, ctx: &mut Context, textures: &WorldTextures, screen: &RenderCoords, border: bool) {
        self.map.draw(ctx, textures, &screen.offset(self.coords), border)
    }

    fn on_tile(&mut self, ctx: &mut Context, battle: BattleEntryRef, player: &mut PlayerCharacter) {
        self.map.on_tile(ctx, battle, player)
    }

}

impl GameWorld for WorldChunkMap {

    fn on_start(&mut self, ctx: &mut Context, music: bool) {
        if let Some(chunk) = self.chunk_mut() {
            chunk.on_start(ctx, music);
        } else {
            warn!("Could not get current chunk {:?}!", self.current);
        }
    }

    fn update(&mut self, ctx: &mut Context, delta: f32, player: &mut PlayerCharacter, battle: BattleEntryRef, warp: &mut Option<WarpDestination>, text_window: &mut TextWindow) {
        if let Some(chunk) = self.chunk_mut() {
            chunk.update(ctx, delta, player, battle, warp, text_window);
        }
    }

    fn draw(&self, ctx: &mut Context, textures: &WorldTextures, screen: &RenderCoords, border: bool) {
        if let Some(chunk) = self.chunk() {
            chunk.draw(ctx, textures, screen, border);
            for connection in &chunk.connections {
                if let Some(chunk) = self.chunks.get(connection) {
                    chunk.draw(ctx, textures, screen, false);
                }
            }
        }        
    }

    fn on_tile(&mut self, ctx: &mut Context, battle: BattleEntryRef, player: &mut PlayerCharacter) {
        if let Some(chunk) = self.chunk_mut() {
            if chunk.in_bounds(player.character.position.coords) {
                chunk.on_tile(ctx, battle, player);
            }
        }
    }

}