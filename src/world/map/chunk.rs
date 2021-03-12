use firecore_world::World;
use firecore_world::map::chunk::WorldChunk;
use firecore_world::map::chunk::world_chunk_map::WorldChunkMap;

use crate::world::GameWorld;
use crate::world::NpcTextures;
use crate::world::RenderCoords;
use crate::world::TileTextures;
use crate::world::gui::map_window_manager::MapWindowManager;
use crate::world::player::Player;

impl GameWorld for WorldChunk {

    fn update(&mut self, delta: f32, player: &mut Player, window_manager: &mut MapWindowManager) {
        self.map.update(delta, player, window_manager);
    }

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, screen: RenderCoords, border: bool) {
        self.map.render(tile_textures, npc_textures, screen.offset(self.x, self.y), border)
    }

    fn on_tile(&mut self, player: &mut Player) {
        self.map.on_tile(player)
    }

    fn input(&mut self, delta: f32, player: &mut Player) {
        self.map.input(delta, player)
    }

}

impl GameWorld for WorldChunkMap {

    fn update(&mut self, delta: f32, player: &mut Player, window_manager: &mut MapWindowManager) {
        self.current_chunk_mut().update(delta, player, window_manager);
    }

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, screen: RenderCoords, border: bool) {
        let current_chunk = self.current_chunk();
        current_chunk.render(tile_textures, npc_textures, screen, border);
        for connection in &current_chunk.connections {
            self.chunks.get(connection).expect("Could not get connected chunk").render(tile_textures, npc_textures, screen, false);
        }
    }

    fn input(&mut self, delta: f32, player: &mut Player) {
        self.current_chunk_mut().input(delta, player)
    }

    fn on_tile(&mut self, player: &mut Player) {
        let current_chunk = self.current_chunk_mut();
        if current_chunk.in_bounds(player.position.local.coords.x, player.position.local.coords.y) {
            current_chunk.on_tile(player);
        }
    }

}