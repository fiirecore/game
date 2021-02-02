use ahash::AHashMap as HashMap;
use macroquad::prelude::info;
use crate::util::input;
use crate::util::texture::Texture;
use crate::audio::music::Music;
use crate::entity::texture::three_way_texture::ThreeWayTexture;
use crate::io::data::Direction;
use crate::util::input::Control;
use crate::util::render::draw_flip;
use crate::util::render::draw_o;
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

    fn on_tile(&mut self, /*player: &mut Player,*/ x: isize, y: isize) {
        let tile_id = self.tile(x, y);

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

        for npc in &mut self.npcs {
            if let Some(trainer) = &npc.trainer {
                if let Some(tracker) = &trainer.tracker {
                    match npc.position.direction {
                        Direction::Up => {
                            if x == npc.position.x {
                                if y > npc.position.y && y <= npc.position.y + tracker.length as isize {
                                    info!("NPC {} Found player at tile {}, {}", npc.identifier.name, x, y);
                                    npc.interact(None, x, y);
                                    //player.frozen = true;
                                    //npc.movement.walk_to_player();
                                }
                            }
                        }
                        Direction::Down => {
                            if x == npc.position.x {
                                if y < npc.position.y && y >= npc.position.y - tracker.length as isize {
                                    info!("NPC {} Found player at tile {}, {}", npc.identifier.name, x, y);
                                    npc.interact(None, x, y);
                                    //player.frozen = true;
                                }
                            }
                        }
                        Direction::Left => {
                            if y == npc.position.y {
                                if x < npc.position.x && x >= npc.position.x - tracker.length as isize {
                                    info!("NPC {} Found player at tile {}, {}", npc.identifier.name, x, y);
                                    npc.interact(None, x, y);
                                    //player.frozen = true;
                                }
                            }
                        }
                        Direction::Right => {
                            if y == npc.position.y {
                                if x > npc.position.x && x <= npc.position.x + tracker.length as isize {
                                    info!("NPC {} Found player at tile {}, {}", npc.identifier.name, x, y);
                                    npc.interact(None, x, y);
                                    //player.frozen = true;
                                }
                            }
                        }
                    }
                }
            }            
        }
    }

    fn render(&self, textures: &HashMap<u16, Texture>, npc_textures: &HashMap<u8, ThreeWayTexture>, screen: RenderCoords, border: bool) {
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
            if let Some(twt) = npc_textures.get(&npc.identifier.sprite) {
                let tuple = twt.of_direction(npc.position.direction.value());
                draw_flip(tuple.0, (npc.position.x << 4) as f32 - screen.x_focus + 1.0, (npc.position.y << 4) as f32 - screen.y_focus - 4.0, tuple.1);
            }            
        }
    }

    fn input(&mut self, _delta: f32, player: &Player) {
        if input::pressed(Control::A) {
            for npc in &mut self.npcs {
                if player.position.x == npc.position.x {
                    match player.position.direction {
                        Direction::Up => {
                            if player.position.y - 1 == npc.position.y {
                                npc.interact(Some(player.position.direction), player.position.x, player.position.y);
                            }
                        },
                        Direction::Down => {
                            if player.position.y + 1 == npc.position.y {
                                npc.interact(Some(player.position.direction), player.position.x, player.position.y);
                            }
                        },
                        _ => {}
                    }
                } else if player.position.y == npc.position.y {
                    match player.position.direction {
                        Direction::Right => {
                            if player.position.x + 1 == npc.position.x {
                                npc.interact(Some(player.position.direction), player.position.x, player.position.y);
                            }
                        },
                        Direction::Left => {
                            if player.position.x - 1 == npc.position.x {
                                npc.interact(Some(player.position.direction), player.position.x, player.position.y);
                            }
                        },
                        _ => {}
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