use firecore_world::map::World;
use firecore_world::map::chunk::WorldChunk;
use firecore_world::map::chunk::map::WorldChunkMap;

use crate::world::{GameWorld, TileTextures, NpcTextures, GuiTextures, RenderCoords};
use crate::world::gui::map_window_manager::MapWindowManager;
use firecore_world::character::player::PlayerCharacter;

impl GameWorld for WorldChunk {

    fn on_start(&self, music: bool) {
        self.map.on_start(music);
    }

    fn update(&mut self, delta: f32, player: &mut PlayerCharacter, window_manager: &mut MapWindowManager) {
        self.map.update(delta, player, window_manager);
    }

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, gui_textures: &GuiTextures, screen: RenderCoords, border: bool) {
        self.map.render(tile_textures, npc_textures, gui_textures, screen.offset(self.coords), border)
    }

    fn on_tile(&mut self, player: &mut PlayerCharacter) {
        self.map.on_tile(player)
    }

    fn input(&mut self, delta: f32, player: &mut PlayerCharacter) {
        self.map.input(delta, player)
    }

}

impl GameWorld for WorldChunkMap {

    fn on_start(&self, music: bool) {
        self.current_chunk().on_start(music);
    }

    fn update(&mut self, delta: f32, player: &mut PlayerCharacter, window_manager: &mut MapWindowManager) {
        self.current_chunk_mut().update(delta, player, window_manager);
    }

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, gui_textures: &GuiTextures, screen: RenderCoords, border: bool) {
        let current_chunk = self.current_chunk();
        current_chunk.render(tile_textures, npc_textures, gui_textures, screen, border);
        for connection in &current_chunk.connections {
            self.chunks.get(connection).expect("Could not get connected chunk").render(tile_textures, npc_textures, gui_textures, screen, false);
        }
    }

    fn input(&mut self, delta: f32, player: &mut PlayerCharacter) {
        self.current_chunk_mut().input(delta, player)
    }

    fn on_tile(&mut self, player: &mut PlayerCharacter) {
        let current_chunk = self.current_chunk_mut();
        if current_chunk.in_bounds(player.position.local.coords) {
            current_chunk.on_tile(player);
        }
    }

}