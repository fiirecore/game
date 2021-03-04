use macroquad::prelude::info;
use crate::util::Coordinate;
use frc_audio::music::Music;
use crate::util::Direction;
use frc_input::{self as input, Control};
use super::npc::manager::MapNpcManager;

use super::MapSize;
use super::MovementId;
use super::NpcTextures;
use super::TileId;
use super::TileTextures;
use super::pokemon::WildEntry;
use super::warp::WarpEntry;
use super::player::Player;
use super::RenderCoords;
use super::World;

pub mod manager;
pub mod set;
pub mod chunk;

// pub mod script_manager;

#[derive(Default)]
pub struct WorldMap {

    pub name: String,
    pub music: Music,

    pub width: MapSize,
    pub height: MapSize,

    pub tile_map: Vec<TileId>,
    pub border_blocks: [u16; 4],
    pub movement_map: Vec<MovementId>,

    pub fly_position: Coordinate,
    // pub draw_color: Option<macroquad::prelude::Color>,
    pub wild: Option<WildEntry>,

    pub warps: Vec<WarpEntry>,
    pub npc_manager: MapNpcManager,
    // pub script_manager: MapScriptManager,

}

impl WorldMap {

    fn tile_row(&self, x: isize, offset: isize) -> u16 {
        self.tile_map[x as usize + offset as usize]
    }

    pub fn after_interact(&mut self, index: usize) {
        self.npc_manager.npcs[index].after_interact(&self.name);
    }

}

impl World for WorldMap {

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        return !(x < 0 || (x as u16) >= self.width || y < 0 || (y as u16) >= self.height);
    }

    fn tile(&self, x: isize, y: isize) -> TileId {
        self.tile_map[x as usize + y as usize * self.width as usize]
    }

    fn walkable(&self, x: isize, y: isize) -> MovementId {
        for npc in &self.npc_manager.npcs {
            if npc.position.coords.y == y && npc.position.coords.x == x {
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
        let tile_id = self.tile(player.position.local.coords.x, player.position.local.coords.y);

        if let Some(wild) = &self.wild {
            if let Some(tiles) = &wild.tiles {
                tiles.iter().for_each(|tile| {
                    if tile_id.eq(tile) {
                        try_wild_battle(wild);
                    }
                });
            } else {
                try_wild_battle(wild);
            }            
        }

        for npc in self.npc_manager.npcs.iter_mut().enumerate() {
            let (npc_index, npc) = npc;
            if let Some(trainer) = &npc.trainer {
                if let Some(mut data) = macroquad::prelude::collections::storage::get_mut::<crate::io::data::player::PlayerData>() {
                    if !std::ops::DerefMut::deref_mut(&mut data).world_status.get_or_create_map_data(&self.name).battled.contains(&npc.identifier.name) {
                        if let Some(tracker) = trainer.tracking_length {
                            let tracker = tracker as isize;
                            match npc.position.direction {
                                Direction::Up => {
                                    if player.position.local.coords.x == npc.position.coords.x {
                                        if player.position.local.coords.y < npc.position.coords.y && player.position.local.coords.y >= npc.position.coords.y - tracker {
                                            // info!("NPC {} Found player at tile {}, {}", npc.identifier.name, player.position.local.coords.x, player.position.local.coords.y);
                                            self.npc_manager.npc_active = Some(npc_index);
                                            npc.interact(None, player);
                                            
                                            //npc.movement.walk_to_player();
                                        }
                                    }
                                }
                                Direction::Down => {
                                    if player.position.local.coords.x == npc.position.coords.x {
                                        if player.position.local.coords.y > npc.position.coords.y && player.position.local.coords.y <= npc.position.coords.y + tracker {
                                            // info!("NPC {} Found player at tile {}, {}", npc.identifier.name, player.position.local.coords.x, player.position.local.coords.y);
                                            self.npc_manager.npc_active = Some(npc_index);
                                            npc.interact(None, player);
                                        }
                                    }
                                }
                                Direction::Left => {
                                    if player.position.local.coords.y == npc.position.coords.y {
                                        if player.position.local.coords.x < npc.position.coords.x && player.position.local.coords.x >= npc.position.coords.x - tracker {
                                            // info!("NPC {} Found player at tile {}, {}", npc.identifier.name, player.position.local.coords.x, player.position.local.coords.y);
                                            self.npc_manager.npc_active = Some(npc_index);
                                            npc.interact(None, player);
                                        }
                                    }
                                }
                                Direction::Right => {
                                    if player.position.local.coords.y == npc.position.coords.y {
                                        if player.position.local.coords.x > npc.position.coords.x && player.position.local.coords.x <= npc.position.coords.x + tracker {
                                            // info!("NPC {} Found player at tile {}, {}", npc.identifier.name, player.position.local.coords.x, player.position.local.coords.y);
                                            self.npc_manager.npc_active = Some(npc_index);
                                            npc.interact(None, player);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }            
        }

        // self.script_manager.on_tile(player);

    }

    fn update(&mut self, delta: f32, player: &mut Player) {
        // self.script_manager.update(delta, player);
    }

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, screen: RenderCoords, border: bool) {
        for yy in screen.top..screen.bottom {
            let y = yy - screen.y_tile_offset;
            let render_y = (yy << 4) as f32 - screen.focus.y; // old = y_tile w/ offset - player x pixel
            
            let row_offset = y * self.width as isize;
            
            for xx in screen.left..screen.right {
                let x = xx - screen.x_tile_offset;
                let render_x = (xx << 4) as f32 - screen.focus.x;

                if !(x < 0 || y < 0 || y >= self.height as isize || x >= self.width as isize) {
                    tile_textures.render_tile(&self.tile_row(x, row_offset), render_x, render_y);
                } else if border {
                    if x % 2 == 0 {
                        if y % 2 == 0 {
                            tile_textures.render_tile(&self.border_blocks[0], render_x, render_y);
                        } else {
                            tile_textures.render_tile(&self.border_blocks[2], render_x, render_y);
                        }
                    } else {
                        if y % 2 == 0 {
                            tile_textures.render_tile(&self.border_blocks[1], render_x, render_y);
                        } else {
                            tile_textures.render_tile(&self.border_blocks[3], render_x, render_y);
                        }
                    }
                }
            }
        }
        for npc in &self.npc_manager.npcs {
            npc.render(npc_textures, &screen);
        }
        // self.script_manager.render(tile_textures, npc_textures, &screen);
    }

    fn input(&mut self, _delta: f32, player: &mut Player) {

        if cfg!(debug_assertions) {
            if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::F4) {
                for npc in &self.npc_manager.npcs {
                    info!("NPC {} is at {}, {}; looking {:?}", &npc.identifier.name, &npc.position.coords.x, &npc.position.coords.y, &npc.position.direction);
                }
            }
        }
        
        if input::pressed(Control::A) {
            for npc_index in 0..self.npc_manager.npcs.len() {
                let npc = &mut self.npc_manager.npcs[npc_index];
                if player.position.local.coords.x == npc.position.coords.x {
                    match player.position.local.direction {
                        Direction::Up => {
                            if player.position.local.coords.y - 1 == npc.position.coords.y {
                                self.npc_manager.npc_active = Some(npc_index);
                                npc.interact(Some(player.position.local.direction), player);
                            }
                        },
                        Direction::Down => {
                            if player.position.local.coords.y + 1 == npc.position.coords.y {
                                self.npc_manager.npc_active = Some(npc_index);
                                npc.interact(Some(player.position.local.direction), player);
                            }
                        },
                        _ => (),
                    }
                } else if player.position.local.coords.y == npc.position.coords.y {
                    match player.position.local.direction {
                        Direction::Right => {
                            if player.position.local.coords.x + 1 == npc.position.coords.x {
                                self.npc_manager.npc_active = Some(npc_index);
                                npc.interact(Some(player.position.local.direction), player);
                            }
                        },
                        Direction::Left => {
                            if player.position.local.coords.x - 1 == npc.position.coords.x {
                                self.npc_manager.npc_active = Some(npc_index);
                                npc.interact(Some(player.position.local.direction), player);
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