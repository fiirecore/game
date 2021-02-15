use ahash::AHashMap as HashMap;
use crate::util::graphics::Texture;
use crate::audio::music::Music;
use crate::entity::Entity;
use crate::world::NpcTextures;
use crate::world::RenderCoords;
use crate::world::World;
use crate::world::map::manager::test_move_code;
use crate::world::player::Player;
use crate::world::warp::WarpEntry;
use super::world_chunk::WorldChunk;

#[derive(Default)]
pub struct WorldChunkMap {

    alive: bool,

    pub(crate) chunks: HashMap<u16, WorldChunk>,
    //pub(crate) current_chunk: &'a WorldChunk,
    //connected_chunks: Vec<&'a WorldChunk>,
    pub(crate) current_chunk: u16,

    pub current_music: Music,

}

impl WorldChunkMap {

    pub fn change_chunk(&mut self, chunk: u16, player: &mut Player) {
        self.current_chunk = chunk;
        {
            let chunk1 = self.chunks.get(&self.current_chunk).unwrap();
            player.position.local.x = player.position.get_x() - chunk1.x;
            player.position.local.y = player.position.get_y() - chunk1.y;
            player.position.x_offset = chunk1.x;
            player.position.y_offset = chunk1.y;
        }
        let music = self.current_chunk().map.music;
        if music != self.current_music {
            self.current_music = music;
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

    fn walkable(&self, x: isize, y: isize) -> u8 {
        let current = self.current_chunk();
        if current.in_bounds(x, y) {
            current.walkable(x, y)
        } else {
            // for connection_id in &current.connections {
            //     let connection = self.chunks.get(connection_id).expect("Could not get connected chunk");
            //     if connection.in_bounds(x, y) {
            //         // To - do: check if walkable here
            //         //self.current_chunk = *connection_id;
            //         // let music = connection.map.music;
            //         // if music != self.current_music {
            //         //     self.current_music = music;
            //         //     context.play_music(self.current_music);
            //         // }
            //         return self.walkable(x, y);
            //     }
            // }
            return 1;
        }        
    }

    fn check_warp(&self, x: isize, y: isize) -> Option<WarpEntry> {
        self.current_chunk().check_warp(x, y)
    }

    fn render(&self, textures: &HashMap<u16, Texture>, npc_textures: &NpcTextures, screen: RenderCoords, border: bool) {
        let current_chunk = self.current_chunk();
        current_chunk.render(textures, npc_textures, screen, border);
        for connection in &current_chunk.connections {
            self.chunks.get(connection).expect("Could not get connected chunk").render(textures, npc_textures, screen, false);
        }
    }

    fn input(&mut self, delta: f32, player: &mut Player) {
        self.current_chunk_mut().input(delta, player)
    }

    fn on_tile(&mut self, player: &mut Player) {
        if self.current_chunk().in_bounds(player.position.local.x, player.position.local.y) {
            self.current_chunk_mut().on_tile(player);
        }
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
