use std::collections::HashMap;
use log::warn;
use opengl_graphics::{GlGraphics, Texture};
use piston_window::Context;
use crate::entity::entity::Ticking;
use crate::entity::texture::three_way_texture::ThreeWayTexture;
use crate::util::map_util::GameMap;
use crate::util::map_util::GameMapDraw;
use crate::{engine::text::TextRenderer, entity::entity::Entity, util::map_traits::MapManager};

use crate::engine::game_context::GameContext;
use crate::util::traits::Loadable;

use crate::game::world::world_map::world_map::WorldMap;
use crate::entity::entities::player::Player;

pub struct WorldMapManager {

    alive: bool,

    pub current_world: WorldMap,
    //pub worlds: HashMap<String, WorldMap>,
    //pub current_world_id: String,

    pub generate_a_battle: bool,

}

impl WorldMapManager {

    pub fn new() -> WorldMapManager {

        WorldMapManager {

            alive: false,

            current_world: WorldMap::new(),

            generate_a_battle: false,

        }
        
    }

    pub fn new_world_map(&mut self) {
        self.current_world = WorldMap::new();
    }

    pub fn get_current_world(&self) -> &WorldMap {
        return &self.current_world;
        //return self.worlds.get(&self.current_world_id).expect("Could not get current world map");
    }

    pub fn get_current_world_mut(&mut self) -> &mut WorldMap {
        return &mut self.current_world;
        //return self.worlds.get_mut(&self.current_world_id).expect("Could not get current world map");
    }

    pub fn reload(&mut self, context: &mut GameContext) {
        self.get_current_world_mut().on_start(context);
    }

    pub fn current_tile_action(&mut self, context: &mut GameContext, tile_id: u16) {
        let current_world = self.get_current_world();
        let mut gen = false;
        if let Some(table) = current_world.get_current_piece().wild_pokemon_table.as_ref() {
            match current_world.get_current_piece().wild_tiles.as_ref() {
                Some(wild_tiles) => {
                    for tile in wild_tiles {
                        if tile.eq(&tile_id) {
                            let rand = context.random.rand_range(0..256) as u8;
                            if rand < table.encounter_rate() {
                                gen = true;
                            }
                        }
                    }
                },
                None => {
                    warn!("Could not find wild tiles for map {}", current_world.get_current_piece().name);
                }
            }            
            
        }
        self.generate_a_battle = gen;
    }

    /*

    pub fn generate_battle22(&mut self, context: &mut GameContext, pokedex: &Pokedex) -> PokemonInstance {
        self.generate_a_battle = false;
        let current_world = self.get_current_world();
        if let Some(_piece) = current_world.pieces.get(&current_world.current_piece_index) {
            let pkmn: Vec<usize> = vec![1, 4, 7, 25];
            let level = 30;
            let id = context.random.rand_range(0..pkmn.len() as u32) as usize;
            return PokemonInstance::generate(pokedex, context, pokedex.pokemon_from_id(pkmn[id]), level, level);
        } else {
            return PokemonInstance::generate(pokedex, context, pokedex.pokemon_from_id(1), 1, 1);
        }
          
    }

    */

    pub fn get_tile_id(&mut self, x: isize, y: isize) -> Option<u16> {
        let current_world = self.get_current_world();
        if let Some(i) = current_world.map_index_at_coordinates(x, y) {
            let map = current_world.pieces.get(&i).unwrap();
            let _x: usize = (x - map.x) as usize;
            let _y: usize = (y - map.y) as usize;
            let tile_id = map.tile_map[_x + _y * map.width as usize];
            return Some(tile_id);
        } else {
            return None;
        }
    }

}



impl MapManager for WorldMapManager {

    fn render_below(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer, textures: &HashMap<u16, Texture>, npc_textures: &HashMap<u8, ThreeWayTexture>, player: &Player) {
        self.get_current_world().draw_bottom_map(ctx, g, textures, npc_textures, player);
        player.render(ctx, g, tr);    
    }

    fn render_above(&self, ctx: &mut Context, g: &mut GlGraphics, _tr: &mut TextRenderer, textures: &HashMap<u16, Texture>, player: &Player) {
        self.get_current_world().draw_top_map(ctx, g, textures, player);
    }

    fn input(&mut self, context: &mut GameContext, player: &Player) {
        self.get_current_world_mut().input(context, player);
    }

    fn on_tile(&mut self, context: &mut GameContext, player: &Player) {
        if let Some(tile_id) = self.get_tile_id(player.coords.x, player.coords.y) {
            self.current_tile_action(context, tile_id);
        }
    }

}

impl Loadable for WorldMapManager {

    fn load(&mut self) {
        
    }

    fn on_start(&mut self, context: &mut GameContext) {
        self.reload(context);
    }

}

impl Entity for WorldMapManager {
    
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