use std::collections::HashMap;

use log::info;
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture;
use piston_window::Context;

use crate::audio::music::Music;
use crate::engine::game_context::GameContext;
use crate::entity::entities::player::Player;
use crate::entity::texture::three_way_texture::ThreeWayTexture;
use crate::io::data::Direction;
use crate::util::render_util::draw_flip;
use crate::util::render_util::draw_o;

use super::ScreenCoords;
use super::World;
use super::npc::NPC;
use super::pokemon::WildEntry;
use super::warp::WarpEntry;

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

    fn walkable(&mut self, _context: &mut GameContext, x: isize, y: isize) -> u8 {
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

    fn on_tile(&mut self, context: &mut GameContext, /*player: &mut Player,*/ x: isize, y: isize) {
        let tile_id = self.tile(x, y);
        if let Some(wild) = &self.wild {
            for tile in &wild.tiles {
                if tile_id.eq(tile) {
                    if (context.random.rand_range(0..256) as u8) < wild.table.encounter_rate() {
                        context.battle_context.wild_battle(&mut context.random, &wild.table);
                    }
                }
            }
        }
        for npc in &self.npcs {
            if let Some(trainer) = &npc.trainer {
                if let Some(tracker) = &trainer.tracker {
                    match npc.position.direction {
                        Direction::Up => {
                            if x == npc.position.x {
                                if y > npc.position.y && y <= npc.position.y + tracker.length as isize {
                                    info!("NPC {} Found player at tile {}, {}", npc.identifier.name, x, y);
                                    //player.frozen = true;
                                    //npc.movement.walk_to_player();
                                }
                            }
                        }
                        Direction::Down => {
                            if x == npc.position.x {
                                if y < npc.position.y && y >= npc.position.y - tracker.length as isize {
                                    info!("NPC {} Found player at tile {}, {}", npc.identifier.name, x, y);
                                    //player.frozen = true;
                                }
                            }
                        }
                        Direction::Left => {
                            if y == npc.position.y {
                                if x < npc.position.x && x >= npc.position.x - tracker.length as isize {
                                    info!("NPC {} Found player at tile {}, {}", npc.identifier.name, x, y);
                                    //player.frozen = true;
                                }
                            }
                        }
                        Direction::Right => {
                            if y == npc.position.y {
                                if x > npc.position.x && x <= npc.position.x + tracker.length as isize {
                                    info!("NPC {} Found player at tile {}, {}", npc.identifier.name, x, y);
                                    //player.frozen = true;
                                }
                            }
                        }
                    }
                }
            }            
        }
    }

    fn render(&self, ctx: &mut Context, g: &mut GlGraphics, textures: &HashMap<u16, Texture>, npc_textures: &HashMap<u8, ThreeWayTexture>, screen: ScreenCoords, border: bool) {
        for yy in screen.y0..screen.y1 {
            let y = yy - screen.offset_y;
            let shift_y = (yy << 4) - screen.focus_y;
            let offset = y * self.width as isize;
            
            for xx in screen.x0..screen.x1 {
                let x = xx - screen.offset_x;
                let shift_x = (xx << 4) - screen.focus_x;
                if !(x < 0 || y < 0 || y >= self.height as isize || x >= self.width as isize) {
                        draw_o(ctx, g, textures.get(&self.tile_row(x, offset)), shift_x, shift_y);             
                } else if border {
                    if x % 2 == 0 {
                        if y % 2 == 0 {
                            draw_o(ctx, g, textures.get(&self.border_blocks[0]), shift_x, shift_y);
                        } else {
                            draw_o(ctx, g, textures.get(&self.border_blocks[2]), shift_x, shift_y);
                        }
                    } else {
                        if y % 2 == 0 {
                            draw_o(ctx, g, textures.get(&self.border_blocks[1]), shift_x, shift_y);
                        } else {
                            draw_o(ctx, g, textures.get(&self.border_blocks[3]), shift_x, shift_y);
                        }
                    }
                }
            }
        }
        for npc in &self.npcs {
            let tuple = npc_textures.get(&npc.identifier.sprite).expect("Could not find NPC texture!").of_direction(npc.position.direction.value());
            draw_flip(ctx, g, tuple.0, (npc.position.x << 4) - screen.focus_x + 1, (npc.position.y << 4) - screen.focus_y - 4, tuple.1);
        }
    }

    fn input(&mut self, context: &mut GameContext, player: &Player) {
        if context.keys[0] == 1 {
            for npc in &mut self.npcs {
                if player.position.x == npc.position.x {
                    match player.position.direction {
                        Direction::Up => {
                            if player.position.y - 1 == npc.position.y {
                                npc.interact(player.position.direction, context);
                            }
                        },
                        Direction::Down => {
                            if player.position.y + 1 == npc.position.y {
                                npc.interact(player.position.direction, context);
                            }
                        },
                        _ => {}
                    }
                } else if player.position.y == npc.position.y {
                    match player.position.direction {
                        Direction::Right => {
                            if player.position.x + 1 == npc.position.x {
                                npc.interact(player.position.direction, context);
                            }
                        },
                        Direction::Left => {
                            if player.position.x - 1 == npc.position.x {
                                npc.interact(player.position.direction, context);
                            }
                        },
                        _ => {}
                    }
                }
                
            }
        }
    }

}