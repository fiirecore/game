use std::collections::HashMap;

use opengl_graphics::Texture;

use crate::entity::entity::Entity;
use crate::entity::texture::three_way_texture::ThreeWayTexture;

use super::ScreenCoords;
use super::World;
use super::warp::WarpEntry;
use super::world_chunk::WorldChunk;

pub struct WorldChunkMap {

    alive: bool,

    pub(crate) chunks: HashMap<u16, WorldChunk>,
    //pub(crate) current_chunk: &'a WorldChunk,
    //connected_chunks: Vec<&'a WorldChunk>,
    pub(crate) current_chunk: u16,

    current_music: u8,

}

impl WorldChunkMap {

    pub fn new() -> Self {

        Self {
            alive: false,
            chunks: HashMap::new(),
            current_chunk: 0,
            current_music: 0,
        }

    }



    pub fn change_chunk(&mut self, chunk: u16) {
        self.current_chunk = chunk;
        let music = self.current_chunk().map.music;
        if music != self.current_music {
            self.current_music = music;
            crate::audio::music::Music::play_music(self.current_music);
        }
    }    

    pub fn chunk_at(&self, x: isize, y: isize) -> Option<(&u16, &WorldChunk)> {
        for chunk in &self.chunks {
            if chunk.1.in_bounds(x, y) {
                return Some(chunk);
            }
        }
        None
    }

    pub fn chunk_id_at(&self, x: isize, y: isize) -> Option<u16> {
        for chunk in &self.chunks {
            if chunk.1.in_bounds(x, y) {
                return Some(*chunk.0);
            }
        }
        None
    }

    pub fn current_chunk(&self) -> &WorldChunk {
        self.chunks.get(&self.current_chunk).expect("Could not get current chunk")
    }

    pub(crate) fn current_chunk_mut(&mut self) -> &mut WorldChunk {
        self.chunks.get_mut(&self.current_chunk).expect("Could not get current chunk")
    }



    pub fn insert(&mut self, index: u16, chunk: WorldChunk) {
        self.chunks.insert(index, chunk);
    }

}

impl World for WorldChunkMap {

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        self.current_chunk().in_bounds(x, y)
    }

    fn tile(&self, x: isize, y: isize) -> u16 {
        let current_chunk = self.current_chunk();
        if let Some(tile) = current_chunk.safe_tile(x, y) {
            return tile;
        } else {
            for connection in &current_chunk.connections {
                let chunk = self.chunks.get(connection).expect("Could not get current chunk");
                if let Some(tile) = chunk.safe_tile(x, y) {
                    return tile;
                }
            }
            if y % 2 == 0 {
                if x % 2 == 0 {
                    current_chunk.map.border_blocks[0]
                } else {
                    current_chunk.map.border_blocks[2]
                }
            } else {
                if x % 2 == 0 {
                    current_chunk.map.border_blocks[1]
                } else {
                    current_chunk.map.border_blocks[3]
                }
            }

        }
    }

    fn walkable(&mut self, x: isize, y: isize) -> u8 {
        let current = self.current_chunk();
        if current.in_bounds(x, y) {
            self.chunks.get_mut(&self.current_chunk).unwrap().walkable(x, y)
        } else {
            for connection in &current.connections {
                if self.chunks.get(connection).expect("Could not get connected chunk").in_bounds(x, y) {
                    // To - do: check if walkable here
                    self.change_chunk(*connection);
                    return self.walkable(x, y);
                }
            }
            return 1;
        }        
    }

    fn check_warp(&self, x: isize, y: isize) -> Option<WarpEntry> {
        self.current_chunk().check_warp(x, y)
    }

    fn render(&self, ctx: &mut piston_window::Context, g: &mut opengl_graphics::GlGraphics, textures: &HashMap<u16, Texture>, npc_textures: &HashMap<u8, ThreeWayTexture>, screen: ScreenCoords, border: bool) {
        let current_chunk = self.current_chunk();
        current_chunk.render(ctx, g, textures, npc_textures, screen, border);
        for connection in &current_chunk.connections {
            self.chunks.get(connection).expect("Could not get connected chunk").render(ctx, g, textures, npc_textures, screen, false);
        }
    }

    fn input(&mut self, context: &mut crate::engine::game_context::GameContext, player: &crate::entity::entities::player::Player) {
        self.current_chunk_mut().input(context, player)
    }

    fn on_tile(&mut self, context: &mut crate::engine::game_context::GameContext, tile_id: u16) {
        self.current_chunk_mut().on_tile(context, tile_id)
    }
    
}

impl Entity for WorldChunkMap {
    fn spawn(&mut self) {
        self.alive = true;
    }

    fn despawn(&mut self) {
        self.alive = false;
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}
