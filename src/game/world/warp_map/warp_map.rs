
use std::collections::HashMap;

use opengl_graphics::GlGraphics;
use opengl_graphics::Texture;
use piston_window::Context;

use crate::engine::game_context::GameContext;
use crate::entity::entities::player::Player;
use crate::entity::texture::three_way_texture::ThreeWayTexture;
use crate::entity::util::direction::Direction;
use crate::game::npc::npc::NPC;
use crate::game::warp::warp_entry::WarpEntry;
use crate::game::world::pokemon::wild_pokemon_table::WildPokemonTable;
use crate::util::map_util::GameMap;
use crate::util::map_util::GameMapDraw;
use crate::util::map_util::screen_coords;
use crate::util::render_util::TEXTURE_SIZE;
use crate::util::render_util::VIEW_HEIGHT;
use crate::util::render_util::VIEW_WIDTH;
use crate::util::render_util::draw;
use crate::util::render_util::draw_flip;

pub struct WarpMap {

    pub music: u8,

    pub width: u16,
    pub height: u16,

    pub tile_map: Vec<u16>,
    pub border_blocks: [u16; 4],
    pub movement_map: Vec<u8>,

    pub warps: Vec<WarpEntry>,
    pub npcs: Vec<NPC>,

    pub wild_tiles: Option<Vec<u16>>,
    pub wild_pokemon_table: Option<Box<dyn WildPokemonTable>>,

}

impl WarpMap {

}

impl GameMap for WarpMap {

    fn tile(&self, x: isize, y: isize) -> u16 {
        self.tile_map[x as usize + y as usize * self.width as usize]
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

    fn input(&mut self, context: &mut GameContext, player: &Player) {
        if context.keys[0] == 1 {
            for npc in &mut self.npcs {
                if player.coords.x == npc.x {
                    match player.direction {
                        Direction::Up => {
                            if player.coords.y - 1 == npc.y {
                                npc.interact(player.direction, context);
                            }
                        },
                        Direction::Down => {
                            if player.coords.y + 1 == npc.y {
                                npc.interact(player.direction, context);
                            }
                        },
                        _ => {}
                    }
                } else if player.coords.y == npc.y {
                    match player.direction {
                        Direction::Right => {
                            if player.coords.x + 1 == npc.x {
                                npc.interact(player.direction, context);
                            }
                        },
                        Direction::Left => {
                            if player.coords.x - 1 == npc.x {
                                npc.interact(player.direction, context);
                            }
                        },
                        _ => {}
                    }
                }
                
            }
        }
    }
    
}

impl GameMapDraw for WarpMap {

    fn draw_bottom_map(&self, ctx: &mut Context, g: &mut GlGraphics, textures: &HashMap<u16, Texture>, npc_textures: &HashMap<u8, ThreeWayTexture>, player: &Player) {

        let (x0, x1, y0, y1) = screen_coords(player);

        for y in y0..y1 {
            for x in x0..x1 {
                let shift_x = (x << 4) - player.focus_x;
                let shift_y = (y << 4) - player.focus_y;
                if !(x < 0 || y < 0 || y >= self.height as isize || x >= self.width as isize) {
                    if let Some(tex) = textures.get(&self.tile(x, y)) {
                        draw(ctx, g, tex, shift_x, shift_y);
                    }                    
                } else if x % 2 == 0 {
                    if y % 2 == 0 {
                        draw(ctx, g, textures.get(&self.border_blocks[0]).unwrap(), shift_x, shift_y);
                    } else {
                        draw(ctx, g, textures.get(&self.border_blocks[2]).unwrap(), shift_x, shift_y);
                    }
                } else {
                    if y % 2 == 0 {
                        draw(ctx, g, textures.get(&self.border_blocks[1]).unwrap(), shift_x, shift_y);
                    } else {
                        draw(ctx, g, textures.get(&self.border_blocks[3]).unwrap(), shift_x, shift_y);
                    }
                }
            }
        }

        for npc in &self.npcs {
            let tuple = npc_textures.get(&npc.sprite).expect("Could not find NPC texture!").of_direction(npc.direction.int_value());
            draw_flip(ctx, g, tuple.0, (npc.x << 4) - player.focus_x + 1, (npc.y << 4) - player.focus_y - 4, tuple.1);
        }

    }

    fn draw_top_map(&self, ctx: &mut Context, g: &mut GlGraphics, textures: &HashMap<u16, Texture>, player: &Player) {
        let x0 = player.focus_x >> 4;
		let x1 = (player.focus_x + (VIEW_WIDTH + TEXTURE_SIZE) as isize) >> 4;
		let y0 = player.focus_y >> 4;
        let y1 = (player.focus_y + (VIEW_HEIGHT + TEXTURE_SIZE) as isize) >> 4;

        for y in y0..y1 {
            for x in x0..x1 {
                if !(x < 0 || y < 0 || y >= self.height as isize || x >= self.width as isize) {
                    let i = self.tile_map[x as usize + y as usize * self.width as usize];
                    draw(ctx, g, textures.get(&i).unwrap(), (x << 4) - player.focus_x, (y << 4) - player.focus_y);
                }
            }
        }
    }

}