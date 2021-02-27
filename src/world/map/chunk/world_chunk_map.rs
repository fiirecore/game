use ahash::AHashMap as HashMap;
use macroquad::prelude::debug;
use crate::util::graphics::Texture;
use crate::audio::music::Music;
use crate::world::NpcTextures;
use crate::world::RenderCoords;
use crate::world::World;
use crate::world::map::manager::test_move_code;
use crate::world::player::Player;
use crate::world::warp::WarpEntry;
use super::WorldChunk;

pub struct WorldChunkMap {

    chunks: HashMap<u16, WorldChunk>,
    current_chunk: u16,

    pub current_music: Music,

}

impl WorldChunkMap {

    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            current_chunk: 2,
            current_music: Music::default(),
        }
    }

    pub fn update_chunk(&mut self, chunk_id: &u16) -> Option<&WorldChunk> {
        if let Some(chunk) = self.chunks.get(chunk_id) {
            self.current_chunk = *chunk_id;
            return Some(chunk);
        } else {
            return None;
        }
    }

    pub fn change_chunk(&mut self, chunk: u16, player: &mut Player) {
        if let Some(chunk1) = self.update_chunk(&chunk) {
            {
                player.position.local.coords.x = player.position.get_x() - chunk1.x;
                player.position.local.coords.y = player.position.get_y() - chunk1.y;
                player.position.offset.x = chunk1.x;
                player.position.offset.y = chunk1.y;
            }
            debug!("Entered chunk: {}", chunk1.map.name);
            let music = self.current_chunk().map.music;
            if music != self.current_music {
                self.current_music = music;
            }
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
            if chunk.1.in_bounds(x - chunk.1.x, y - chunk.1.y) {
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

    pub fn connections(&self) -> Vec<(&u16, &WorldChunk)> {
        self.current_chunk().connections.iter().map(|connection| (connection, self.chunks.get(connection).expect("Could not get connected chunks"))).collect()
    }



    pub fn insert(&mut self, index: u16, chunk: WorldChunk) {
        self.chunks.insert(index, chunk);
    }

    pub(crate) fn tiles(&self) -> Vec<crate::world::TileId> {
        let mut tiles = Vec::new();
        for chunk in self.chunks.values() {
            for tile_id in &chunk.map.tile_map {
                if !tiles.contains(tile_id) {
                    tiles.push(*tile_id);
                }
            }
            for tile_id in &chunk.map.border_blocks {
                if !tiles.contains(tile_id) {
                    tiles.push(*tile_id);
                }
            }
        }
        return tiles;
    }



    pub fn walk_connections(&mut self, player: &mut Player, x: isize, y: isize) -> u8 {
        let mut move_code = 1;
        let mut chunk = None;
        for connection in self.connections() {
            let x = x - connection.1.x;
            let y = y - connection.1.y;
            if connection.1.in_bounds(x, y) {
                move_code = connection.1.walkable(x, y);
                chunk = Some(*connection.0);
            }
        }
        if let Some(chunk) = chunk {
            if test_move_code(move_code, false) {
                self.change_chunk(chunk, player);   
            }
        }
        return move_code;
    }

}

impl World for WorldChunkMap {

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        self.current_chunk().in_bounds(x, y)
    }

    fn tile(&self, x: isize, y: isize) -> u16 {
        if let Some(tile) = self.current_chunk().safe_tile(x, y) {
            return tile;
        } else {
            let current_chunk = self.current_chunk();
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

    fn walkable(&self, x: isize, y: isize) -> u8 {
        self.current_chunk().walkable(x, y)  
    }

    fn check_warp(&self, x: isize, y: isize) -> Option<WarpEntry> {
        self.current_chunk().check_warp(x, y)
    }

    fn update(&mut self, delta: f32, player: &mut Player) {
        self.current_chunk_mut().update(delta, player);
    }

    fn render(&self, tile_textures: &HashMap<u16, Texture>, npc_textures: &NpcTextures, screen: RenderCoords, border: bool) {
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