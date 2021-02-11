use ahash::AHashMap as HashMap;
use macroquad::prelude::info;
use crate::util::graphics::draw_rect;
use crate::util::input;
use crate::util::graphics::Texture;
use crate::audio::music::Music;
use crate::io::data::Direction;
use crate::util::input::Control;
use crate::util::graphics::draw_flip;
use crate::util::graphics::draw_o;
use super::NpcTextures;
use super::npc::NPC;
use super::pokemon::WildEntry;
use super::warp::WarpEntry;
use super::player::Player;
use super::RenderCoords;
use super::World;

pub mod manager;

pub mod set {
    pub mod world_map_set;
    pub mod world_map_set_manager;
}

pub mod chunk {
    pub mod world_chunk;
    pub mod world_chunk_map;
}

#[derive(Default)]
pub struct WorldMap {

    pub name: String,
    pub music: Music,

    pub width: u16,
    pub height: u16,

    pub tile_map: Vec<u16>,
    pub border_blocks: [u16; 4],
    pub movement_map: Vec<u8>,

    pub wild: Option<WildEntry>,
    pub warps: Vec<WarpEntry>,
    pub npcs: Vec<NPC>,
    pub npc_active: Option<usize>,

}

impl WorldMap {

    fn tile_row(&self, x: isize, offset: isize) -> u16 {
        self.tile_map[x as usize + offset as usize]
    }

}

impl World for WorldMap {

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        return !(x < 0 || (x as u16) >= self.width || y < 0 || (y as u16) >= self.height);
    }

    fn tile(&self, x: isize, y: isize) -> u16 {
        self.tile_map[x as usize + y as usize * self.width as usize]
    }

    fn walkable(&self, x: isize, y: isize) -> u8 {
        for npc in &self.npcs {
            if npc.position.y == y && npc.position.x == x {
                return 1;
            }
        }
        self.movement_map[x as usize + y as usize * self.width as usize]
    }

    fn check_warp(&self, x: isize, y: isize) -> Option<WarpEntry> {
        for warp in &self.warps {
            if warp.x == x {
                if warp.y == y {
                    return Some(warp.clone());
                }
            }
        }
        return None;
    }

    fn on_tile(&mut self, player: &mut Player) {
        let tile_id = self.tile(player.position.local.x, player.position.local.y);

        if let Some(wild) = &self.wild {
            if let Some(tiles) = &wild.tiles {
                for tile in tiles {
                    if tile_id.eq(tile) {
                        try_wild_battle(wild);
                    }
                }
            } else {
                try_wild_battle(wild);
            }            
        }

        for npc_index in 0..self.npcs.len() {
            let npc = &mut self.npcs[npc_index];
            if let Some(trainer) = &npc.trainer {
                if let Some(tracker) = trainer.tracking_length {
                    let tracker = tracker as isize;
                    match npc.position.direction {
                        Direction::Up => {
                            if player.position.local.x == npc.position.x {
                                if player.position.local.y < npc.position.y && player.position.local.y >= npc.position.y - tracker {
                                    info!("NPC {} Found player at tile {}, {}", npc.identifier.name, player.position.local.x, player.position.local.y);
                                    self.npc_active = Some(npc_index);
                                    npc.interact(None, player.position.local.x, player.position.local.y);
                                    player.freeze();
                                    //npc.movement.walk_to_player();
                                }
                            }
                        }
                        Direction::Down => {
                            if player.position.local.x == npc.position.x {
                                if player.position.local.y > npc.position.y && player.position.local.y <= npc.position.y + tracker {
                                    info!("NPC {} Found player at tile {}, {}", npc.identifier.name, player.position.local.x, player.position.local.y);
                                    self.npc_active = Some(npc_index);
                                    npc.interact(None, player.position.local.x, player.position.local.y);
                                    player.freeze();
                                }
                            }
                        }
                        Direction::Left => {
                            if player.position.local.y == npc.position.y {
                                if player.position.local.x < npc.position.x && player.position.local.x >= npc.position.x - tracker {
                                    info!("NPC {} Found player at tile {}, {}", npc.identifier.name, player.position.local.x, player.position.local.y);
                                    self.npc_active = Some(npc_index);
                                    npc.interact(None, player.position.local.x, player.position.local.y);
                                    player.freeze();
                                }
                            }
                        }
                        Direction::Right => {
                            if player.position.local.y == npc.position.y {
                                if player.position.local.x > npc.position.x && player.position.local.x <= npc.position.x + tracker {
                                    info!("NPC {} Found player at tile {}, {}", npc.identifier.name, player.position.local.x, player.position.local.y);
                                    self.npc_active = Some(npc_index);
                                    npc.interact(None, player.position.local.x, player.position.local.y);
                                    player.freeze();
                                }
                            }
                        }
                    }
                }
            }            
        }
    }

    fn render(&self, textures: &HashMap<u16, Texture>, npc_textures: &NpcTextures, screen: RenderCoords, border: bool) {
        for yy in screen.top..screen.bottom {
            let y = yy - screen.y_tile_offset;
            let render_y = (yy << 4) as f32 - screen.y_focus; // old = y_tile w/ offset - player x pixel
            
            let row_offset = y * self.width as isize;
            
            for xx in screen.left..screen.right {
                let x = xx - screen.x_tile_offset;
                let render_x = (xx << 4) as f32 - screen.x_focus;

                if !(x < 0 || y < 0 || y >= self.height as isize || x >= self.width as isize) {
                    draw_o(textures.get(&self.tile_row(x, row_offset)), render_x, render_y);             
                } else if border {
                    if x % 2 == 0 {
                        if y % 2 == 0 {
                            draw_o(textures.get(&self.border_blocks[0]), render_x, render_y);
                        } else {
                            draw_o(textures.get(&self.border_blocks[2]), render_x, render_y);
                        }
                    } else {
                        if y % 2 == 0 {
                            draw_o(textures.get(&self.border_blocks[1]), render_x, render_y);
                        } else {
                            draw_o(textures.get(&self.border_blocks[3]), render_x, render_y);
                        }
                    }
                }
            }
        }
        for npc in &self.npcs {
            let x = ((npc.position.x + screen.x_tile_offset) << 4) as f32 - screen.x_focus + npc.position.x_offset;
            let y = ((npc.position.y - 1 + screen.y_tile_offset) << 4) as f32 - screen.y_focus + npc.position.y_offset;
            if let Some(twt) = npc_textures.get(&npc.identifier.npc_type) {
                let tuple = twt.of_direction(npc.position.direction);
                draw_flip(tuple.0, x, y, tuple.1);
            } else {
                draw_rect([1.0, 0.0, 0.0, 1.0], x, y + crate::util::TILE_SIZE as f32, 16, 16);
            }      
        }
    }

    fn input(&mut self, _delta: f32, player: &Player) {
        if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::F4) {
            for npc in &self.npcs {
                info!("NPC {} is at {}, {}; looking {:?}", &npc.identifier.name, &npc.position.x, &npc.position.y, &npc.position.direction);
            }
        }
        
        if input::pressed(Control::A) {
            for npc_index in 0..self.npcs.len() {
                let npc = &mut self.npcs[npc_index];
                if player.position.local.x == npc.position.x {
                    match player.position.local.direction {
                        Direction::Up => {
                            if player.position.local.y - 1 == npc.position.y {
                                self.npc_active = Some(npc_index);
                                npc.interact(Some(player.position.local.direction), player.position.local.x, player.position.local.y);
                            }
                        },
                        Direction::Down => {
                            if player.position.local.y + 1 == npc.position.y {
                                self.npc_active = Some(npc_index);
                                npc.interact(Some(player.position.local.direction), player.position.local.x, player.position.local.y);
                            }
                        },
                        _ => (),
                    }
                } else if player.position.local.y == npc.position.y {
                    match player.position.local.direction {
                        Direction::Right => {
                            if player.position.local.x + 1 == npc.position.x {
                                self.npc_active = Some(npc_index);
                                npc.interact(Some(player.position.local.direction), player.position.local.x, player.position.local.y);
                            }
                        },
                        Direction::Left => {
                            if player.position.local.x - 1 == npc.position.x {
                                self.npc_active = Some(npc_index);
                                npc.interact(Some(player.position.local.direction), player.position.local.x, player.position.local.y);
                            }
                        },
                        _ => (),
                    }
                }
                
            }
        }
    }

}

fn try_wild_battle(wild: &WildEntry) {
    if macroquad::rand::gen_range(0, 255) < wild.table.encounter_rate() {
        crate::util::battle_data::wild_battle(&wild.table);
    }
}