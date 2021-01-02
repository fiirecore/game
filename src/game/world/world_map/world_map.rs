use crate::entity::texture::three_way_texture::ThreeWayTexture;
use crate::game::warp::warp_entry::WarpEntry;
use crate::util::map_util::GameMap;
use crate::util::map_util::GameMapDraw;
use crate::util::map_util::screen_coords;
use crate::util::render_util::draw_flip;
use crate::{audio::music::Music, engine::game_context::GameContext, util::traits::Loadable};
use piston_window::Context;
use std::collections::HashMap;
use opengl_graphics::{GlGraphics, Texture};
//use opengl_graphics::{GlGraphics, Texture};


use crate::game::world::world_map::world_map_piece::WorldMapPiece;

use crate::entity::entities::player::Player;

use crate::util::render_util::draw;

pub struct WorldMap {

    pub pieces: HashMap<usize, WorldMapPiece>,
    pub current_piece_index: usize,
    
    current_music: Music,

}

impl WorldMap {

    pub fn new() -> WorldMap {

        WorldMap {

            pieces: HashMap::new(),
            current_piece_index: 0,

            current_music: Music::Route2,

        }

    }

    pub fn get_current_piece(&self) -> &WorldMapPiece {
        return self.pieces.get(&self.current_piece_index).expect("Could not get current world piece.");
    }

    pub fn get_current_piece_mut(&mut self) -> &mut WorldMapPiece {
        return self.pieces.get_mut(&self.current_piece_index).expect("Could not get current world piece.");
    }

    pub fn map_index_at_coordinates(&self, x: isize, y: isize) -> Option<usize> {
        for i in self.pieces.iter() {
            let map = i.1;
            if y >= map.y && y < map.y + map.height as isize {
                if x >= map.x && x < map.x + map.width as isize {
                    return Some(*i.0);
                }
            }
        }
        None
    }

    pub fn update_current_map_piece(&mut self, i: usize) {
        if i != self.current_piece_index {
            self.current_piece_index = i;
            let new_music = Music::from_int(self.pieces[&self.current_piece_index].music);
            if let Some(music) = new_music {
                if self.current_music != music {
                    self.current_music = music;
                    self.play_current_music();
                }
            }            
        }
    }

    fn play_current_music(&mut self) {
        music::set_volume(0.2);
        music::play_music(&self.current_music, music::Repeat::Forever);    
    }

    pub fn walkable(&mut self, x: isize, y: isize) -> u8 {
        match self.map_index_at_coordinates(x, y) {
            Some(i) => {
                self.update_current_map_piece(i);
                match self.pieces.get(&i) {
                    Some(map) => {
                        return map.movement_map[(x - map.x) as usize + (y - map.y) as usize * map.width as usize];
                    }
                    None => {
                        return 1;
                    }
                }
                
            }
            None => {
                return 1;
            }
        }
        
    }

    pub fn find_on_load(&mut self, x: isize, y: isize) {
        self.current_piece_index = self.map_index_at_coordinates(x, y).unwrap_or(0);
    }

}

impl GameMap for WorldMap {
    
    fn tile(&self, x: isize, y: isize) -> u16 {
        return self.get_current_piece().tile(x, y);
    }

    fn check_warp(&self, x: isize, y: isize) -> Option<WarpEntry> {
        match self.map_index_at_coordinates(x, y) {
            Some(i) => {
                let map = self.pieces.get(&i).unwrap();
                return map.check_warp(x - map.x, y - map.y);
            }
            None => {
                return None;
            }
        }
    }

    fn input(&mut self, context: &mut GameContext, player: &Player) {
        if let Some(piece) = self.pieces.get_mut(&self.current_piece_index) {
            piece.input(context, player);
        }
    }
}

impl GameMapDraw for WorldMap {

    fn draw_bottom_map(
        &self,
        ctx: &mut Context,
        g: &mut GlGraphics,
        textures: &HashMap<u16, Texture>,
        npc_textures: &HashMap<u8, ThreeWayTexture>,
        player: &Player,
    ) {

        let (x0, x1, y0, y1) = screen_coords(player);

        if let Some(c_map) = self.pieces.get(&self.current_piece_index) {
            
            for yy in y0..y1 {
                for xx in x0..x1 {
                    let x = xx - c_map.x;
                    let y = yy - c_map.y;
                    if !(x < 0 || y < 0 || y >= c_map.height as isize || x >= c_map.width as isize) {
                        draw(ctx, g, textures.get(&c_map.tile_map[x as usize + y as usize * c_map.width as usize]).unwrap(), ((xx << 4) - player.focus_x) as isize, ((yy << 4) - player.focus_y) as isize);
                    } else if x % 2 == 0 {
                        if y % 2 == 0 {
                            draw(ctx, g, textures.get(&c_map.border_blocks[0]).unwrap(), ((xx << 4) - player.focus_x) as isize, ((yy << 4) - player.focus_y) as isize);
                        } else {
                            draw(ctx, g, textures.get(&c_map.border_blocks[2]).unwrap(), ((xx << 4) - player.focus_x) as isize, ((yy << 4) - player.focus_y) as isize);
                        }
                    } else {
                        if y % 2 == 0 {
                            draw(ctx, g, textures.get(&c_map.border_blocks[1]).unwrap(), ((xx << 4) - player.focus_x) as isize, ((yy << 4) - player.focus_y) as isize);
                        } else {
                            draw(ctx, g, textures.get(&c_map.border_blocks[3]).unwrap(), ((xx << 4) - player.focus_x) as isize, ((yy << 4) - player.focus_y) as isize);
                        }
                    }
                }
            }
    
            for map_index in &c_map.connections {
                let map = self.pieces.get(&map_index);
                if let Some(map) = map {
                    for yy in y0..y1 {
                        for xx in x0..x1 {
                            let x = xx - map.x;
                            let y = yy - map.y;
                            if !(x < 0 || y < 0 || y >= map.height as isize || x >= map.width as isize) {
                                draw(ctx, g, textures.get(&map.tile(x, y)).unwrap(), ((xx << 4) - player.focus_x) as isize, ((yy << 4) - player.focus_y) as isize);
                            }
                        }
                    }

                    for npc in &map.npcs {
                        let tuple = npc_textures.get(&npc.sprite).expect("Could not find NPC texture!").of_direction(npc.direction.int_value());
                        draw_flip(ctx, g, tuple.0, (npc.x << 4) - player.focus_x + 1, (npc.y << 4) - player.focus_y - 4, tuple.1);
                    }

                }         

            }

            for npc in &c_map.npcs {
                let tuple = npc_textures.get(&npc.sprite).expect("Could not find NPC texture!").of_direction(npc.direction.int_value());
                draw_flip(ctx, g, tuple.0, (npc.x << 4) - player.focus_x + 1, (npc.y << 4) - player.focus_y - 4, tuple.1);
            }

        }

    }

    fn draw_top_map(
        &self,
        ctx: &mut Context,
        g: &mut GlGraphics,
        textures: &HashMap<u16, Texture>,
        player: &Player,
    ) {

        let (x0, x1, y0, y1) = screen_coords(player);

        if let Some(c_map) = self.pieces.get(&self.current_piece_index) {

            for yy in y0..y1 {
                for xx in x0..x1 {
                    let x = xx - c_map.x;
                    let y = yy - c_map.y;
                    if !(x < 0 || y < 0 || y >= c_map.height as isize || x >= c_map.width as isize) {
                        draw(ctx, g, textures.get(&c_map.tile(x, y)).unwrap(), ((xx << 4) - player.focus_x) as isize, ((yy << 4) - player.focus_y) as isize);
                    }
                }
            }

        }

    }
}

impl Loadable for WorldMap {

    fn load(&mut self) {

    }

    fn on_start(&mut self, _context: &mut GameContext) {
        self.current_music = Music::from_int(self.pieces[&self.current_piece_index].music).unwrap_or(Music::Title);
        self.play_current_music();
    }  

}
