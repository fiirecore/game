use log::info;
use log::warn;
use opengl_graphics::{GlGraphics, Texture};
use piston_window::Context;
use crate::entity::entity::Ticking;
use crate::entity::texture::three_way_texture::ThreeWayTexture;
use crate::util::map_util::GameMap;
use crate::util::map_util::GameMapDraw;
use crate::{engine::game_context::GameContext, entity::entity::Entity, util::{map_traits::MapManager, traits::Loadable}};
use crate::engine::text::TextRenderer;
use crate::audio::music::Music;

use std::collections::HashMap;

use crate::entity::entities::player::Player;
use crate::game::world::warp_map::warp_map_set::WarpMapSet;

pub struct WarpMapManager {

    alive: bool,

    pub map_sets: HashMap<String, WarpMapSet>,
    pub current_map_set_id: String,

    current_music: Music,

    pub generate_a_battle: bool,

}

impl WarpMapManager {

    pub fn new() -> WarpMapManager {

        WarpMapManager {

            alive: false,
            
            map_sets: HashMap::new(),
            current_map_set_id: String::new(),

            current_music: Music::Route2,

            generate_a_battle: false,

        }

    }

    pub fn load_set(&mut self, id: &String, num: usize) {
        self.current_map_set_id = id.clone();
        self.load_set_num(num);
    }

    pub fn load_set_num(&mut self, num: usize) {
        self.map_sets.get_mut(&self.current_map_set_id).unwrap().current_map_index = num;
    }

    pub fn reload(&mut self) {
        let map_set = self.map_sets.get(&self.current_map_set_id).unwrap();
        self.current_music = Music::from_int(map_set.maps.get(&map_set.current_map_index).unwrap().music).unwrap_or(Music::Route2);
        self.play_current_music();
    }

    pub fn play_current_music(&self) {
        music::set_volume(0.2);
        music::play_music(&self.current_music, music::Repeat::Forever);
    }

    pub fn get_tile_id(&mut self, x: isize, y: isize) -> Option<u16> {
        let map = self.current_map_set().current_map();
        if x < 0 || y < 0 || x as u16 >= map.width || y as u16 >= map.height {
            return None;
        } else {
            let tile_id = map.tile_map[x as usize + y as usize * map.width as usize];
            return Some(tile_id);
        }        
    }

    pub fn current_tile_action(&mut self, context: &mut GameContext, tile_id: u16) {
        let map = self.current_map_set().current_map();
        let mut gen = false;
        if let Some(table) = map.wild_pokemon_table.as_ref() {
            match map.wild_tiles.as_ref() {
                Some(wild_tiles) => {
                    for tile in wild_tiles {
                        if tile.eq(&tile_id) {
                            let rand = context.random.rand_range(0..256) as u8;
                            if rand < table.encounter_rate() {
                                gen = true;
                                info!("Generating battle");
                            }
                        }
                    }
                },
                None => {
                    warn!("Could not find wild tiles for warp map {}", self.current_map_set_id);
                }
            }            
            
        }
        self.generate_a_battle = gen;
    }

    pub fn current_map_set(&self) -> &WarpMapSet {
        return self.map_sets.get(&self.current_map_set_id).expect("Could not get current warp map set!");
    }

    pub fn current_map_set_mut(&mut self) -> &mut WarpMapSet {
        return self.map_sets.get_mut(&self.current_map_set_id).expect("Could not get current warp map set!");
    }

}

impl MapManager for WarpMapManager {

    fn render_below(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer, textures: &HashMap<u16, Texture>, npc_textures: &HashMap<u8, ThreeWayTexture>, player: &Player) {
        self.current_map_set().current_map().draw_bottom_map(ctx, g, textures, npc_textures, player);
        player.render(ctx, g, tr);
    }

    fn render_above(&self, ctx: &mut Context, g: &mut GlGraphics, _tr: &mut TextRenderer, textures: &HashMap<u16, Texture>, player: &Player) {
        self.current_map_set().current_map().draw_top_map(ctx, g, textures, player);
    }

    fn input(&mut self, context: &mut GameContext, player: &Player) {
        self.current_map_set_mut().current_map_mut().input(context, player);
    }    

    fn on_tile(&mut self, context: &mut GameContext, player: &Player) {
        if let Some(tile_id) = self.get_tile_id(player.coords.x, player.coords.y) {
            self.current_tile_action(context, tile_id);
        }
    }

}

impl Loadable for WarpMapManager {

    fn load(&mut self) {
        
    }

    fn on_start(&mut self, _context: &mut GameContext) {
        self.reload();
    }

}

impl Entity for WarpMapManager {

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